// ned is a command line text editor ("new ed")
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        args[1].clone()
    } else {
        "buffer.txt".to_string()
    };

    let mut lines: Vec<String> = Vec::new();

    // Load existing file if it exists
    if fs::metadata(&filename).is_ok() {
        if let Ok(file) = File::open(&filename) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    lines.push(l);
                }
            }
        }
        println!("Loaded {} lines from {}", lines.len(), filename);
    } else {
        println!("Creating new file: {}", filename);
    }

    println!("ned: editing {} (type lines, end with single . on its own line to save/exit)", filename);

    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let trimmed = input.trim_end_matches(&['\r', '\n'][..]);

        if trimmed == "." {
            break;
        }

        lines.push(trimmed.to_string());
    }

    // Save the file
    match File::create(&filename) {
        Ok(mut file) => {
            for line in &lines {
                if let Err(e) = writeln!(file, "{}", line) {
                    eprintln!("Write error: {}", e);
                    return;
                }
            }
            println!("Saved {}", filename);
        }
        Err(e) => eprintln!("Could not save file: {}", e),
    }
}
