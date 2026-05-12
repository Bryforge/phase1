use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn normalize_path(path: &Path) -> String {
    let mut out = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(part) => out.push(part),
            _ => {}
        }
    }

    out.to_string_lossy().replace('\\', "/")
}

fn markdown_targets(doc_path: &str, text: &str) -> BTreeSet<String> {
    let mut targets = BTreeSet::new();
    let base_dir = Path::new(doc_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));

    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b']' && i + 1 < bytes.len() && bytes[i + 1] == b'(' {
            let start = i + 2;
            if let Some(end_rel) = text[start..].find(')') {
                let raw = &text[start..start + end_rel];
                let href = raw.split_whitespace().next().unwrap_or("").trim();

                if !href.is_empty()
                    && !href.starts_with("http://")
                    && !href.starts_with("https://")
                    && !href.starts_with('#')
                    && !href.starts_with("mailto:")
                {
                    let href = href.split('#').next().unwrap_or(href);

                    let normalized = if href.starts_with("base1/")
                        || href.starts_with("docs/")
                        || href.starts_with("scripts/")
                        || href.starts_with("tests/")
                    {
                        normalize_path(Path::new(href))
                    } else {
                        normalize_path(&base_dir.join(href))
                    };

                    targets.insert(normalized);
                }

                i = start + end_rel + 1;
                continue;
            }
        }

        i += 1;
    }

    targets
}

fn root_public_base1_docs() -> Vec<String> {
    let mut paths = Vec::new();

    for entry in fs::read_dir("base1").expect("base1 directory exists") {
        let entry = entry.expect("read base1 entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        let is_public_surface = name.starts_with("RECOVERY_USB_")
            || name.starts_with("LIBREBOOT_")
            || matches!(
                name,
                "SECURITY_MODEL.md"
                    | "HARDWARE_TARGETS.md"
                    | "PHASE1_COMPATIBILITY.md"
                    | "ROADMAP.md"
                    | "NETWORK_LOCKDOWN_DRY_RUN.md"
            );

        if is_public_surface {
            paths.push(format!("base1/{name}"));
        }
    }

    paths.sort();
    paths
}

fn os_public_base1_docs() -> Vec<String> {
    let mut paths = Vec::new();
    let dir = Path::new("docs/os");

    if !dir.exists() {
        return paths;
    }

    for entry in fs::read_dir(dir).expect("docs/os readable") {
        let entry = entry.expect("read docs/os entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        let is_public_base1_surface = name.starts_with("BASE1_")
            || name.contains("ROLLBACK_METADATA")
            || name.contains("INSTALLER")
            || name.contains("RECOVERY")
            || name.contains("IMAGE_BUILDER");

        if is_public_base1_surface {
            paths.push(format!("docs/os/{name}"));
        }
    }

    paths.sort();
    paths
}

fn release_public_base1_docs() -> Vec<String> {
    let release_dir = Path::new("docs/base1/releases");
    let mut paths = Vec::new();

    if !release_dir.exists() {
        return paths;
    }

    for entry in fs::read_dir(release_dir).expect("docs/base1/releases readable") {
        let entry = entry.expect("read release entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if name.starts_with("RELEASE_BASE1_") {
            paths.push(format!("docs/base1/releases/{name}"));
        }
    }

    paths.sort();
    paths
}

#[test]
fn root_readme_links_all_base1_public_surfaces() {
    let text = read("README.md");
    let targets = markdown_targets("README.md", &text);
    let mut missing = Vec::new();

    for path in root_public_base1_docs()
        .into_iter()
        .chain(os_public_base1_docs())
        .chain(release_public_base1_docs())
    {
        if !targets.contains(&path) {
            missing.push(path);
        }
    }

    assert!(
        missing.is_empty(),
        "README.md is missing public Base1 links:\n{}",
        missing.join("\n")
    );
}

#[test]
fn base1_readme_links_all_base1_public_surfaces() {
    let text = read("base1/README.md");
    let targets = markdown_targets("base1/README.md", &text);
    let mut missing = Vec::new();

    for path in root_public_base1_docs() {
        if !targets.contains(&path) {
            missing.push(path);
        }
    }

    assert!(
        missing.is_empty(),
        "base1/README.md is missing public Base1 links:\n{}",
        missing.join("\n")
    );
}

#[test]
fn release_indexes_link_all_base1_release_notes() {
    let indexes = ["docs/base1/releases/README.md", "docs/releases/README.md"];
    let mut all_targets = BTreeSet::new();

    for index in indexes {
        if let Ok(text) = fs::read_to_string(index) {
            all_targets.extend(markdown_targets(index, &text));
        }
    }

    let mut missing = Vec::new();

    for path in release_public_base1_docs() {
        if !all_targets.contains(&path) {
            missing.push(path);
        }
    }

    assert!(
        missing.is_empty(),
        "release indexes are missing Base1 release links:\n{}",
        missing.join("\n")
    );
}
