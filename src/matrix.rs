use std::io::{self, IsTerminal, Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_SECONDS: u64 = 10;
const DEFAULT_WIDTH: usize = 44;
const DEFAULT_HEIGHT: usize = 24;
const MIN_WIDTH: usize = 16;
const MAX_WIDTH: usize = 160;
const MIN_HEIGHT: usize = 8;
const MAX_HEIGHT: usize = 80;

const BINARY_CHARS: &[u8] = b"0101010101010110";
const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
const ALPHA_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SYMBOL_CHARS: &[u8] = b"01<>[]{}()/\\|*-+=#@%&ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GlyphSet {
    Binary,
    Hex,
    Alpha,
    Symbols,
}

impl GlyphSet {
    fn chars(self) -> &'static [u8] {
        match self {
            Self::Binary => BINARY_CHARS,
            Self::Hex => HEX_CHARS,
            Self::Alpha => ALPHA_CHARS,
            Self::Symbols => SYMBOL_CHARS,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MatrixConfig {
    duration: Option<Duration>,
    frame_delay: Duration,
    density: u8,
    glyphs: GlyphSet,
    tail_min: usize,
    tail_max: usize,
    color: bool,
    hud: bool,
    input: bool,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            duration: Some(Duration::from_secs(DEFAULT_SECONDS)),
            frame_delay: Duration::from_millis(45),
            density: 24,
            glyphs: GlyphSet::Symbols,
            tail_min: 4,
            tail_max: 18,
            color: color_enabled(),
            hud: true,
            input: true,
        }
    }
}

#[derive(Clone, Debug)]
struct Column {
    head: isize,
    speed: u64,
    tail: usize,
    glitch: u8,
}

pub fn run(args: &[String]) {
    let config = match parse_args(args) {
        Ok(Some(config)) => config,
        Ok(None) => {
            print_help();
            return;
        }
        Err(err) => {
            println!("matrix: {err}");
            print_help();
            return;
        }
    };

    let mut width = terminal_dim("COLUMNS", DEFAULT_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
    let mut height = terminal_dim("LINES", DEFAULT_HEIGHT).clamp(MIN_HEIGHT, MAX_HEIGHT);
    if width.is_multiple_of(2) && width > MIN_WIDTH {
        width -= 1;
    }
    if height > 2 {
        height -= 1;
    }

    let input_raw = RawMode::enter(config.input);
    let interactive_quit = input_raw.active();
    let effective_duration = if config.duration.is_none() && !interactive_quit {
        Some(Duration::from_secs(DEFAULT_SECONDS))
    } else {
        config.duration
    };

    let _screen = ScreenGuard::enter();
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut rng = Lcg::new(seed());
    let field_height = if config.hud && height > 10 { height - 1 } else { height };
    let end = effective_duration.map(|duration| Instant::now() + duration);
    let mut columns = build_columns(width, field_height, &mut rng, config);
    let mut frame = 0u64;
    let start = Instant::now();

    draw_intro(&mut stdout, width, height, config, interactive_quit, effective_duration);

    loop {
        if let Some(end) = end {
            if Instant::now() >= end {
                break;
            }
        }
        if interactive_quit && user_requested_quit(&mut stdin) {
            break;
        }

        render_frame(&mut stdout, &mut columns, field_height, frame, &mut rng, config);
        if config.hud && height > 10 && frame.is_multiple_of(6) {
            draw_hud(
                &mut stdout,
                width,
                height,
                start.elapsed(),
                effective_duration,
                interactive_quit,
                config,
            );
        }
        let _ = stdout.flush();
        frame = frame.saturating_add(1);
        thread::sleep(config.frame_delay);
    }
}

fn build_columns(width: usize, height: usize, rng: &mut Lcg, config: MatrixConfig) -> Vec<Column> {
    (0..width)
        .map(|_| Column {
            head: -(rng.range(height.max(1) as u64) as isize),
            speed: 1 + rng.range(4),
            tail: config.tail_min + rng.range((config.tail_max - config.tail_min + 1) as u64) as usize,
            glitch: rng.range(100) as u8,
        })
        .collect()
}

fn render_frame(
    stdout: &mut io::Stdout,
    columns: &mut [Column],
    height: usize,
    frame: u64,
    rng: &mut Lcg,
    config: MatrixConfig,
) {
    for (idx, column) in columns.iter_mut().enumerate() {
        if !frame.is_multiple_of(column.speed) {
            continue;
        }
        let x = idx + 1;
        let head = column.head;
        let trail = column.tail as isize;
        let erase = head - trail - 1;

        if (0..height as isize).contains(&erase) {
            print_at(stdout, erase as usize + 1, x, None, ' ');
        }

        for depth in 0..=trail {
            let row = head - depth;
            if !(0..height as isize).contains(&row) {
                continue;
            }
            let color = color_for_depth(depth, trail, column.glitch, config.color);
            let ch = rng.char(config.glyphs);
            print_at(stdout, row as usize + 1, x, color, ch);
        }

        if rng.chance(config.density as u64) {
            let flash_row = rng.range(height as u64) as usize + 1;
            print_at(stdout, flash_row, x, color_for_depth(0, trail, column.glitch, config.color), rng.char(config.glyphs));
        }

        column.head += 1;
        let reset_after = height as isize + rng.range(height as u64).max(1) as isize;
        if column.head - trail > reset_after {
            column.head = -(rng.range(height as u64).max(1) as isize);
            column.speed = 1 + rng.range(4);
            column.tail = config.tail_min + rng.range((config.tail_max - config.tail_min + 1) as u64) as usize;
            column.glitch = rng.range(100) as u8;
        }
    }
}

fn color_for_depth(depth: isize, tail: isize, glitch: u8, color: bool) -> Option<&'static str> {
    if !color {
        return None;
    }
    if glitch > 95 && depth == 0 {
        Some("97")
    } else if depth == 0 {
        Some("92")
    } else if depth < tail / 3 {
        Some("32")
    } else if depth < (tail * 2) / 3 {
        Some("2;32")
    } else {
        Some("90")
    }
}

fn print_at(stdout: &mut io::Stdout, row: usize, col: usize, color: Option<&str>, ch: char) {
    if let Some(color) = color {
        let _ = write!(stdout, "\x1b[{row};{col}H\x1b[{color}m{ch}\x1b[0m");
    } else {
        let _ = write!(stdout, "\x1b[{row};{col}H{ch}");
    }
}

fn draw_intro(
    stdout: &mut io::Stdout,
    width: usize,
    height: usize,
    config: MatrixConfig,
    interactive_quit: bool,
    effective_duration: Option<Duration>,
) {
    let status = if interactive_quit {
        "press q to exit"
    } else {
        "auto-exit timer active"
    };
    let duration = match effective_duration {
        Some(duration) => format!("{}s", duration.as_secs()),
        None => "until q".to_string(),
    };
    let title = format!(" PHASE1 MATRIX // {status} // duration={duration} ");
    let row = height.saturating_div(2).max(1);
    let col = width.saturating_sub(title.chars().count()).saturating_div(2).max(1);
    let color = if config.color { Some("92") } else { None };
    for (offset, ch) in title.chars().enumerate() {
        print_at(stdout, row, col + offset, color, ch);
    }
    let _ = stdout.flush();
    thread::sleep(Duration::from_millis(500));
    let _ = write!(stdout, "\x1b[2J");
}

fn draw_hud(
    stdout: &mut io::Stdout,
    width: usize,
    height: usize,
    elapsed: Duration,
    duration: Option<Duration>,
    interactive_quit: bool,
    config: MatrixConfig,
) {
    let remaining = duration
        .map(|total| total.saturating_sub(elapsed).as_secs().to_string())
        .unwrap_or_else(|| "∞".to_string());
    let quit = if interactive_quit { "q=quit" } else { "timer" };
    let hud = format!(
        " phase1 matrix | {quit} | left={remaining}s | density={} | speed={}ms ",
        config.density,
        config.frame_delay.as_millis()
    );
    let mut clipped = hud.chars().take(width).collect::<String>();
    while clipped.chars().count() < width {
        clipped.push(' ');
    }
    let _ = write!(stdout, "\x1b[{height};1H\x1b[0m{clipped}");
}

fn user_requested_quit(stdin: &mut io::StdinLock<'_>) -> bool {
    let mut buf = [0u8; 16];
    match stdin.read(&mut buf) {
        Ok(0) => false,
        Ok(n) => buf[..n]
            .iter()
            .any(|byte| matches!(*byte, b'q' | b'Q' | 3 | 27)),
        Err(_) => false,
    }
}

fn parse_args(args: &[String]) -> Result<Option<MatrixConfig>, String> {
    let mut config = MatrixConfig::default();
    let mut idx = 0;
    while idx < args.len() {
        let arg = args[idx].as_str();
        match arg {
            "help" | "--help" | "-h" => return Ok(None),
            "forever" | "infinite" | "until-key" => config.duration = None,
            "--mono" => config.color = false,
            "--no-hud" => config.hud = false,
            "--no-input" => config.input = false,
            "--binary" => config.glyphs = GlyphSet::Binary,
            "--hex" => config.glyphs = GlyphSet::Hex,
            "--alpha" => config.glyphs = GlyphSet::Alpha,
            "--symbols" => config.glyphs = GlyphSet::Symbols,
            _ if arg.starts_with("--duration=") => {
                config.duration = parse_duration(value_after_equals(arg)?)?;
            }
            "--duration" | "-d" => {
                idx += 1;
                config.duration = parse_duration(args.get(idx).ok_or("missing duration value")?)?;
            }
            _ if arg.starts_with("--speed=") => {
                config.frame_delay = parse_millis(value_after_equals(arg)?)?;
            }
            "--speed" | "-s" => {
                idx += 1;
                config.frame_delay = parse_millis(args.get(idx).ok_or("missing speed value")?)?;
            }
            _ if arg.starts_with("--density=") => {
                config.density = parse_u8(value_after_equals(arg)?, 1, 90, "density")?;
            }
            "--density" => {
                idx += 1;
                config.density = parse_u8(args.get(idx).ok_or("missing density value")?, 1, 90, "density")?;
            }
            _ if arg.starts_with("--tail=") => {
                let tail = parse_usize(value_after_equals(arg)?, 2, 60, "tail")?;
                config.tail_min = (tail / 3).max(2);
                config.tail_max = tail.max(config.tail_min);
            }
            "--tail" => {
                idx += 1;
                let tail = parse_usize(args.get(idx).ok_or("missing tail value")?, 2, 60, "tail")?;
                config.tail_min = (tail / 3).max(2);
                config.tail_max = tail.max(config.tail_min);
            }
            _ if arg.starts_with("--chars=") => {
                config.glyphs = parse_glyphs(value_after_equals(arg)?)?;
            }
            "--chars" => {
                idx += 1;
                config.glyphs = parse_glyphs(args.get(idx).ok_or("missing chars value")?)?;
            }
            _ if arg.chars().all(|ch| ch.is_ascii_digit()) => {
                config.duration = parse_duration(arg)?;
            }
            _ => return Err(format!("unknown option '{arg}'")),
        }
        idx += 1;
    }
    Ok(Some(config))
}

fn parse_duration(raw: &str) -> Result<Option<Duration>, String> {
    let seconds = raw
        .parse::<u64>()
        .map_err(|_| format!("invalid duration '{raw}'"))?;
    if seconds == 0 {
        Ok(None)
    } else {
        Ok(Some(Duration::from_secs(seconds.clamp(1, 300))))
    }
}

fn parse_millis(raw: &str) -> Result<Duration, String> {
    let millis = raw
        .parse::<u64>()
        .map_err(|_| format!("invalid speed '{raw}'"))?;
    Ok(Duration::from_millis(millis.clamp(15, 250)))
}

fn parse_u8(raw: &str, min: u8, max: u8, name: &str) -> Result<u8, String> {
    raw.parse::<u8>()
        .map(|value| value.clamp(min, max))
        .map_err(|_| format!("invalid {name} '{raw}'"))
}

fn parse_usize(raw: &str, min: usize, max: usize, name: &str) -> Result<usize, String> {
    raw.parse::<usize>()
        .map(|value| value.clamp(min, max))
        .map_err(|_| format!("invalid {name} '{raw}'"))
}

fn parse_glyphs(raw: &str) -> Result<GlyphSet, String> {
    match raw {
        "binary" | "bin" => Ok(GlyphSet::Binary),
        "hex" => Ok(GlyphSet::Hex),
        "alpha" | "alnum" => Ok(GlyphSet::Alpha),
        "symbols" | "ops" => Ok(GlyphSet::Symbols),
        _ => Err(format!("unknown character set '{raw}'")),
    }
}

fn value_after_equals(raw: &str) -> Result<&str, String> {
    raw.split_once('=')
        .map(|(_, value)| value)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("missing value for '{raw}'"))
}

fn print_help() {
    println!("usage: matrix [seconds|0|forever] [options]");
    println!("  q exits cleanly when running in an interactive terminal");
    println!("  0, forever, infinite, until-key  run until q is pressed");
    println!("  --duration <seconds>             duration, clamped to 1-300; 0 means until q");
    println!("  --speed <ms>                     frame delay, clamped to 15-250");
    println!("  --density <1-90>                 random flash density");
    println!("  --tail <2-60>                    maximum rain trail length");
    println!("  --chars binary|hex|alpha|symbols glyph theme");
    println!("  --mono                           disable ANSI colors");
    println!("  --no-hud                         hide status line");
    println!("  --no-input                       timer-only mode");
}

fn terminal_dim(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(default)
}

fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0xC0FFEE)
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
        && std::env::var("PHASE1_NO_COLOR").ok().as_deref() != Some("1")
}

struct ScreenGuard;

impl ScreenGuard {
    fn enter() -> Self {
        print!("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H");
        let _ = io::stdout().flush();
        Self
    }
}

impl Drop for ScreenGuard {
    fn drop(&mut self) {
        print!("\x1b[0m\x1b[?25h\x1b[?1049l");
        let _ = io::stdout().flush();
    }
}

struct RawMode {
    original: Option<String>,
}

impl RawMode {
    fn enter(enabled: bool) -> Self {
        if !enabled || !io::stdin().is_terminal() {
            return Self { original: None };
        }
        let Ok(output) = Command::new("stty")
            .arg("-g")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
        else {
            return Self { original: None };
        };
        if !output.status.success() {
            return Self { original: None };
        }
        let original = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if original.is_empty() {
            return Self { original: None };
        }
        let status = Command::new("stty")
            .args(["-icanon", "-echo", "min", "0", "time", "0"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if matches!(status, Ok(status) if status.success()) {
            Self {
                original: Some(original),
            }
        } else {
            Self { original: None }
        }
    }

    fn active(&self) -> bool {
        self.original.is_some()
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        if let Some(original) = &self.original {
            let _ = Command::new("stty")
                .arg(original)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed ^ 0x9E3779B97F4A7C15)
    }

    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    fn range(&mut self, upper: u64) -> u64 {
        if upper == 0 {
            0
        } else {
            self.next() % upper
        }
    }

    fn char(&mut self, glyphs: GlyphSet) -> char {
        let chars = glyphs.chars();
        chars[(self.next() as usize) % chars.len()] as char
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_args, terminal_dim, GlyphSet};
    use std::time::Duration;

    #[test]
    fn terminal_dim_falls_back() {
        assert_eq!(terminal_dim("PHASE1_TEST_NO_SUCH_DIM", 44), 44);
    }

    #[test]
    fn parses_forever_and_options() {
        let args = [
            "0".to_string(),
            "--speed".to_string(),
            "20".to_string(),
            "--density=40".to_string(),
            "--chars".to_string(),
            "hex".to_string(),
            "--tail=12".to_string(),
            "--mono".to_string(),
        ];
        let config = parse_args(&args).unwrap().unwrap();
        assert_eq!(config.duration, None);
        assert_eq!(config.frame_delay, Duration::from_millis(20));
        assert_eq!(config.density, 40);
        assert_eq!(config.glyphs, GlyphSet::Hex);
        assert!(!config.color);
        assert_eq!(config.tail_max, 12);
    }

    #[test]
    fn clamps_duration_and_speed() {
        let args = ["999".to_string(), "--speed=1".to_string()];
        let config = parse_args(&args).unwrap().unwrap();
        assert_eq!(config.duration, Some(Duration::from_secs(300)));
        assert_eq!(config.frame_delay, Duration::from_millis(15));
    }

    #[test]
    fn help_returns_none() {
        let args = ["--help".to_string()];
        assert!(parse_args(&args).unwrap().is_none());
    }
}
