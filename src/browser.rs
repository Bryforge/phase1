use std::io;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::kernel::VERSION;

const MAX_DOWNLOAD_BYTES: &str = "2097152";
const MAX_RENDER_CHARS: usize = 14_000;
const MAX_SOURCE_CHARS: usize = 24_000;
const FETCH_TIMEOUT: Duration = Duration::from_secs(15);
const CONNECT_TIMEOUT: &str = "5";
const USER_AGENT_PREFIX: &str = "phase1-browser";

#[derive(Clone, Debug)]
struct FetchResponse {
    status: u16,
    content_type: String,
    effective_url: String,
    body: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Link {
    label: String,
    url: String,
}

#[derive(Clone, Debug, Default)]
struct RenderedPage {
    title: String,
    text: String,
    links: Vec<Link>,
}

pub struct Browser;

impl Browser {
    pub fn new() -> Self {
        Self
    }

    pub fn browse(&self, raw_url: &str) -> String {
        let url = raw_url.trim();
        if url.is_empty() || matches!(url, "about" | "about:blank" | "help" | "--help" | "-h") {
            return self.about();
        }
        if url == "phase1" || url.contains("Bryforge/phase1") || url.contains("bryforge/phase1") {
            return self.phase1_page();
        }
        if matches!(url, "links" | "--links") {
            return "usage: browser <http-url|https-url|host|phase1|about>\nexample: browser https://example.com".to_string();
        }

        let normalized_url = match normalize_url(url) {
            Ok(url) => url,
            Err(err) => return format!("browser: {err}"),
        };

        match self.fetch_url(&normalized_url) {
            Ok(response) => self.render_response(&response),
            Err(err) => format!("browser: {err}"),
        }
    }

    fn about(&self) -> String {
        format!(
            "phase1 browser v{}\n\nmode:\n  terminal reader with guarded HTTP/HTTPS fetching\n\ncommands:\n  browser about\n  browser phase1\n  browser https://example.com\n  browser example.com\n\nsafety:\n  schemes        : http, https only\n  credentials    : blocked in URLs\n  connect timeout: {}s\n  request timeout: {}s\n  max download   : 2 MiB\n  renderer       : strips script/style/svg/canvas and lists links\n\nnotes:\n  JavaScript and cookies are intentionally not executed or persisted.\n  Turn SHIELD off and TRUST HOST on before using host-backed browsing.",
            VERSION,
            CONNECT_TIMEOUT,
            FETCH_TIMEOUT.as_secs()
        )
    }

    fn phase1_page(&self) -> String {
        format!(
            "phase1 v{}\nAdvanced Operator Kernel for a terminal-first virtual OS console.\n\nquick starts:\n  help\n  cat readme.txt\n  sysinfo\n  security\n  lang support\n  theme list\n  update protocol\n  browser https://example.com\n\nsecurity:\n  browser/network host access remains gated behind SHIELD off + TRUST HOST on.",
            VERSION
        )
    }

    fn fetch_url(&self, url: &str) -> io::Result<FetchResponse> {
        let max_time = FETCH_TIMEOUT.as_secs().to_string();
        let user_agent = format!("{USER_AGENT_PREFIX}/{VERSION}");
        let mut cmd = Command::new("curl");
        cmd.args([
            "--location",
            "--silent",
            "--show-error",
            "--compressed",
            "--connect-timeout",
            CONNECT_TIMEOUT,
            "--max-time",
            &max_time,
            "--max-redirs",
            "5",
            "--max-filesize",
            MAX_DOWNLOAD_BYTES,
            "--proto",
            "=http,https",
            "--user-agent",
            &user_agent,
            "--write-out",
            "\n\nPHASE1_BROWSER_META:%{http_code}\t%{content_type}\t%{url_effective}\n",
            url,
        ]);

        match run_with_timeout(cmd, FETCH_TIMEOUT + Duration::from_secs(2)) {
            Ok(output) if output.status.success() => parse_curl_response(output, url),
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                let message = if !stderr.trim().is_empty() {
                    stderr.trim().to_string()
                } else if !stdout.trim().is_empty() {
                    strip_curl_meta(&stdout).trim().to_string()
                } else {
                    format!("request failed with status {}", output.status)
                };
                Err(io::Error::other(message))
            }
            Err(err) => Err(err),
        }
    }

    fn render_response(&self, response: &FetchResponse) -> String {
        let rendered = self.render_html(&response.body, &response.effective_url);
        if response.status >= 400 {
            return format!(
                "browser: HTTP {}\nurl : {}\ntype: {}\n\n{}",
                response.status,
                response.effective_url,
                fallback_content_type(&response.content_type),
                first_chars(&rendered.text, MAX_RENDER_CHARS)
            );
        }

        let title = if rendered.title.trim().is_empty() {
            "untitled".to_string()
        } else {
            rendered.title.trim().to_string()
        };
        let mut out = format!(
            "phase1 browser // {}\nurl    : {}\nstatus : HTTP {}\ntype   : {}\n\n{}",
            title,
            response.effective_url,
            response.status,
            fallback_content_type(&response.content_type),
            first_chars(&rendered.text, MAX_RENDER_CHARS)
        );

        if !rendered.links.is_empty() {
            out.push_str("\n\nlinks\n");
            for (idx, link) in rendered.links.iter().take(25).enumerate() {
                let label = if link.label.trim().is_empty() {
                    "link"
                } else {
                    link.label.trim()
                };
                out.push_str(&format!("  [{}] {} -> {}\n", idx + 1, label, link.url));
            }
            if rendered.links.len() > 25 {
                out.push_str(&format!(
                    "  ... {} more links omitted\n",
                    rendered.links.len() - 25
                ));
            }
        }

        out
    }

    fn render_html(&self, html: &str, base_url: &str) -> RenderedPage {
        let mut page = RenderedPage::default();
        let mut text = String::with_capacity(html.len().min(MAX_SOURCE_CHARS));
        let mut chars = html.chars().peekable();
        let mut skip_tag: Option<String> = None;
        let mut in_title = false;
        let mut active_link: Option<usize> = None;

        while let Some(ch) = chars.next() {
            if ch == '<' {
                let tag = read_tag(&mut chars);
                let tag_trimmed = tag.trim();
                if tag_trimmed.starts_with("!--") {
                    continue;
                }

                let closing = tag_trimmed.starts_with('/');
                let name = tag_name(tag_trimmed);

                if closing {
                    if skip_tag.as_deref() == Some(name.as_str()) {
                        skip_tag = None;
                        continue;
                    }
                    if name == "title" {
                        in_title = false;
                        continue;
                    }
                    if name == "a" {
                        if let Some(index) = active_link.take() {
                            text.push_str(&format!(" [{}]", index + 1));
                        }
                        continue;
                    }
                    if is_block_tag(&name) {
                        push_newline(&mut text);
                    }
                    continue;
                }

                if is_skip_tag(&name) {
                    skip_tag = Some(name);
                    continue;
                }
                if skip_tag.is_some() {
                    continue;
                }
                if name == "title" {
                    in_title = true;
                    continue;
                }
                if name == "br" {
                    push_newline(&mut text);
                    continue;
                }
                if name == "li" {
                    push_newline(&mut text);
                    text.push_str("- ");
                    continue;
                }
                if is_heading_tag(&name) {
                    push_newline(&mut text);
                    text.push_str(match name.as_str() {
                        "h1" => "# ",
                        "h2" => "## ",
                        _ => "### ",
                    });
                    continue;
                }
                if name == "a" {
                    if let Some(href) = attr_value(tag_trimmed, "href") {
                        if let Some(url) = resolve_link(base_url, &decode_entities(&href)) {
                            page.links.push(Link {
                                label: String::new(),
                                url,
                            });
                            active_link = Some(page.links.len() - 1);
                        }
                    }
                    continue;
                }
                if name == "img" {
                    if let Some(alt) = attr_value(tag_trimmed, "alt") {
                        let alt = decode_entities(&alt);
                        if !alt.trim().is_empty() {
                            text.push_str("[image: ");
                            text.push_str(alt.trim());
                            text.push(']');
                        }
                    }
                    continue;
                }
                if is_block_tag(&name) {
                    push_newline(&mut text);
                }
                continue;
            }

            if skip_tag.is_some() {
                continue;
            }
            if in_title {
                page.title.push(ch);
                continue;
            }
            text.push(ch);
            if let Some(index) = active_link {
                if let Some(link) = page.links.get_mut(index) {
                    link.label.push(ch);
                }
            }
        }

        page.title = cleanup_inline(&decode_entities(&page.title));
        page.text = cleanup_text(&decode_entities(&text));
        for link in &mut page.links {
            link.label = cleanup_inline(&decode_entities(&link.label));
        }
        page.links.retain(|link| !link.url.trim().is_empty());
        page.links
            .dedup_by(|a, b| a.url == b.url && a.label == b.label);

        if page.text.trim().is_empty() {
            page.text = "browser: fetched page but no readable text was found".to_string();
        }
        page
    }

    #[cfg(test)]
    fn render_text(&self, html: &str) -> String {
        self.render_html(html, "https://phase1.local/").text
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_url(raw: &str) -> Result<String, String> {
    let url = raw.trim();
    if url.is_empty() {
        return Err("empty URL".to_string());
    }
    if url.chars().any(char::is_control) || url.chars().any(char::is_whitespace) {
        return Err("URL must not contain whitespace or control characters".to_string());
    }

    let normalized = if has_scheme(url) {
        url.to_string()
    } else if looks_like_host(url) {
        format!("https://{url}")
    } else {
        return Err("only http:// and https:// URLs are allowed".to_string());
    };

    let lower = normalized.to_ascii_lowercase();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) {
        return Err("only http:// and https:// URLs are allowed".to_string());
    }
    if url_authority(&normalized).is_some_and(|authority| authority.contains('@')) {
        return Err(
            "URL credentials are blocked; do not put usernames, passwords, or tokens in browser URLs"
                .to_string(),
        );
    }
    match url_authority(&normalized) {
        Some(authority) if !authority.trim().is_empty() => Ok(normalized),
        _ => Err("URL host is missing".to_string()),
    }
}

fn has_scheme(url: &str) -> bool {
    url.find("://").is_some_and(|idx| idx > 0)
}

fn looks_like_host(raw: &str) -> bool {
    raw.contains('.') || raw == "localhost"
}

fn url_authority(url: &str) -> Option<&str> {
    let (_, rest) = url.split_once("://")?;
    Some(rest.split(['/', '?', '#']).next().unwrap_or(rest))
}

fn parse_curl_response(output: Output, requested_url: &str) -> io::Result<FetchResponse> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some((body, meta)) = stdout.rsplit_once("PHASE1_BROWSER_META:") else {
        return Ok(FetchResponse {
            status: 200,
            content_type: "unknown".to_string(),
            effective_url: requested_url.to_string(),
            body: stdout.to_string(),
        });
    };

    let mut parts = meta.trim().splitn(3, '\t');
    let status = parts
        .next()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(0);
    let content_type = parts.next().unwrap_or("unknown").trim().to_string();
    let effective_url = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(requested_url)
        .to_string();

    Ok(FetchResponse {
        status,
        content_type,
        effective_url,
        body: body.trim_start_matches(['\r', '\n']).to_string(),
    })
}

fn strip_curl_meta(raw: &str) -> String {
    raw.rsplit_once("PHASE1_BROWSER_META:")
        .map(|(body, _)| body.to_string())
        .unwrap_or_else(|| raw.to_string())
}

fn read_tag<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut tag = String::new();
    for ch in chars.by_ref() {
        if ch == '>' {
            break;
        }
        if tag.len() < 4096 {
            tag.push(ch);
        }
    }
    tag
}

fn tag_name(tag: &str) -> String {
    tag.trim_start_matches('/')
        .trim_start_matches('!')
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

fn attr_value(tag: &str, wanted: &str) -> Option<String> {
    let mut rest = tag;
    loop {
        let lower = rest.to_ascii_lowercase();
        let needle = format!("{wanted}=");
        let Some(pos) = lower.find(&needle) else {
            return None;
        };
        rest = &rest[pos + needle.len()..];
        let rest = rest.trim_start();
        if let Some(stripped) = rest.strip_prefix('"') {
            return stripped.split_once('"').map(|(value, _)| value.to_string());
        }
        if let Some(stripped) = rest.strip_prefix('\'') {
            return stripped
                .split_once('\'')
                .map(|(value, _)| value.to_string());
        }
        return Some(
            rest.split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches('/')
                .to_string(),
        );
    }
}

fn is_skip_tag(name: &str) -> bool {
    matches!(
        name,
        "script" | "style" | "noscript" | "svg" | "canvas" | "iframe"
    )
}

fn is_heading_tag(name: &str) -> bool {
    matches!(name, "h1" | "h2" | "h3")
}

fn is_block_tag(name: &str) -> bool {
    matches!(
        name,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "body"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "tbody"
            | "td"
            | "tfoot"
            | "th"
            | "thead"
            | "tr"
            | "ul"
    )
}

fn push_newline(text: &mut String) {
    if !text.ends_with('\n') {
        text.push('\n');
    }
}

fn decode_entities(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '&' {
            out.push(ch);
            continue;
        }

        let mut entity = String::new();
        while let Some(&next) = chars.peek() {
            chars.next();
            if next == ';' {
                break;
            }
            if entity.len() > 16 {
                out.push('&');
                out.push_str(&entity);
                out.push(next);
                entity.clear();
                break;
            }
            entity.push(next);
        }

        if entity.is_empty() {
            continue;
        }
        match decode_entity(&entity) {
            Some(decoded) => out.push(decoded),
            None => {
                out.push('&');
                out.push_str(&entity);
                out.push(';');
            }
        }
    }
    out
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" | "#39" => Some('\''),
        "nbsp" => Some(' '),
        raw if raw.starts_with("#x") || raw.starts_with("#X") => u32::from_str_radix(&raw[2..], 16)
            .ok()
            .and_then(char::from_u32),
        raw if raw.starts_with('#') => raw[1..].parse::<u32>().ok().and_then(char::from_u32),
        _ => None,
    }
}

fn cleanup_text(input: &str) -> String {
    let mut out = String::new();
    let mut previous_blank = true;
    for line in input.lines().map(cleanup_inline) {
        if line.is_empty() {
            if !previous_blank {
                out.push('\n');
            }
            previous_blank = true;
        } else {
            out.push_str(&line);
            out.push('\n');
            previous_blank = false;
        }
    }
    out.trim().to_string()
}

fn cleanup_inline(input: &str) -> String {
    let mut out = String::new();
    let mut saw_space = false;
    for ch in input.trim().chars() {
        if ch.is_whitespace() {
            saw_space = true;
        } else {
            if saw_space && !out.is_empty() {
                out.push(' ');
            }
            out.push(ch);
            saw_space = false;
        }
    }
    out
}

fn resolve_link(base_url: &str, href: &str) -> Option<String> {
    let href = href.trim();
    if href.is_empty() || href.starts_with("javascript:") || href.starts_with("mailto:") {
        return None;
    }
    if href.starts_with("http://") || href.starts_with("https://") {
        return Some(href.to_string());
    }
    if let Some(stripped) = href.strip_prefix("//") {
        let scheme = base_url
            .split_once("://")
            .map(|(scheme, _)| scheme)
            .unwrap_or("https");
        return Some(format!("{scheme}://{stripped}"));
    }
    let origin = url_origin(base_url)?;
    if href.starts_with('/') {
        return Some(format!("{origin}{href}"));
    }
    if href.starts_with('#') {
        return Some(format!("{base_url}{href}"));
    }
    Some(format!("{}/{}", url_directory(base_url), href))
}

fn url_origin(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let authority = rest.split('/').next().unwrap_or(rest);
    Some(format!("{scheme}://{authority}"))
}

fn url_directory(url: &str) -> String {
    let Some(origin) = url_origin(url) else {
        return url.trim_end_matches('/').to_string();
    };
    let path = url.strip_prefix(&origin).unwrap_or("");
    if path.is_empty() || path == "/" {
        return origin;
    }
    let dir = path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
    format!("{}{}", origin, dir.trim_end_matches('/'))
}

fn fallback_content_type(content_type: &str) -> &str {
    if content_type.trim().is_empty() {
        "unknown"
    } else {
        content_type.trim()
    }
}

fn first_chars(input: &str, limit: usize) -> String {
    if input.chars().count() <= limit {
        return input.to_string();
    }
    let mut out = input.chars().take(limit).collect::<String>();
    out.push_str("\n...[truncated by phase1 browser]");
    out
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_url, Browser};

    #[test]
    fn browser_strips_script_text() {
        let text =
            Browser::new().render_text("<h1>Hello</h1><script>bad()</script><p>World &amp; ok</p>");
        assert!(text.contains("Hello"));
        assert!(text.contains("World & ok"));
        assert!(!text.contains("bad"));
    }

    #[test]
    fn browser_extracts_links_and_decodes_numeric_entities() {
        let page = Browser::new().render_html(
            "<title>T</title><p>A &#35;1 <a href=\"/docs\">Docs</a></p>",
            "https://example.com/root/index.html",
        );
        assert_eq!(page.title, "T");
        assert!(page.text.contains("A #1 Docs [1]"));
        assert_eq!(page.links[0].url, "https://example.com/docs");
    }

    #[test]
    fn browser_normalizes_hosts_and_blocks_credentials() {
        assert_eq!(normalize_url("example.com").unwrap(), "https://example.com");
        assert!(normalize_url("https://user:pass@example.com").is_err());
        assert!(normalize_url("file:///etc/passwd").is_err());
    }
}
