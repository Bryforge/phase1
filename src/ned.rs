use std::io::{self, Write};

use crate::kernel::Vfs;

pub fn edit(vfs: &mut Vfs, filename: &str) {
    if filename.trim().is_empty() {
        println!("Usage: ned <file>");
        return;
    }

    let mut buffer = match vfs.cat(filename) {
        Ok(content) => content,
        Err(_) => String::new(),
    };

    println!("ned: editing {}", filename);
    println!("Commands: single '.' or ':wq' saves and exits; ':q' exits without saving.");
    if !buffer.is_empty() {
        println!("--- current content ---");
        print!("{}", buffer);
        if !buffer.ends_with('\n') {
            println!();
        }
        println!("--- append below ---");
    }

    loop {
        print!("ned> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("ned: input error");
            return;
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);

        match trimmed {
            "." | ":wq" => match vfs.write_file(filename, &buffer, false) {
                Ok(_) => println!("Saved {}", filename),
                Err(e) => println!("Save failed: {}", e),
            },
            ":q" => {
                println!("Exited without saving");
                return;
            }
            _ => {
                buffer.push_str(trimmed);
                buffer.push('\n');
                continue;
            }
        }

        return;
    }
}
