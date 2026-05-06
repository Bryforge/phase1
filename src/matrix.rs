use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MATRIX_CHARS: &[u8] = b"01010123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn run(args: &[String]) {
    let seconds = args.first().and_then(|arg| arg.parse::<u64>().ok()).unwrap_or(10).clamp(1, 60);
    let width = terminal_dim("COLUMNS", 44).clamp(16, 120);
    let height = terminal_dim("LINES", 24).clamp(8, 60);
    let end = Instant::now() + Duration::from_secs(seconds);
    let mut rng = Lcg::new(seed());
    let mut drops = vec![0usize; width];

    print!("\x1b[?25l\x1b[2J\x1b[H");
    let _ = io::stdout().flush();

    while Instant::now() < end {
        for (x, y) in drops.iter_mut().enumerate() {
            if rng.chance(10) {
                *y = 0;
            }

            if *y < height {
                print_at(*y + 1, x + 1, "32", rng.char());
                if *y > 0 {
                    print_at(*y, x + 1, "90", rng.char());
                }
                *y += 1;
            }
        }
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(50));
    }

    print!("\x1b[?25h\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

fn print_at(row: usize, col: usize, color: &str, ch: char) {
    print!("\x1b[{row};{col}H\x1b[{color}m{ch}\x1b[0m");
}

fn terminal_dim(name: &str, default: usize) -> usize {
    std::env::var(name).ok().and_then(|raw| raw.parse().ok()).unwrap_or(default)
}

fn seed() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0xC0FFEE)
}

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed ^ 0x9E3779B97F4A7C15)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    fn char(&mut self) -> char {
        MATRIX_CHARS[(self.next() as usize) % MATRIX_CHARS.len()] as char
    }
}

#[cfg(test)]
mod tests {
    use super::terminal_dim;

    #[test]
    fn terminal_dim_falls_back() {
        assert_eq!(terminal_dim("PHASE1_TEST_NO_SUCH_DIM", 44), 44);
    }
}
