use std::io;

use crate::kernel::Vfs;

pub fn edit(vfs: &mut Vfs, filename: &str) {
    if filename.trim().is_empty() {
        println!("Usage: ned <file>");
        return;
    }

    let content = match vfs.cat(filename) {
        Ok(c) => c,
        Err(_) => String::new(),
    };

    println!("ned: editing {} (type lines, end with single . on new line to save/exit, or :q to quit)", filename);

    let mut builder = content;

    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        let line = line.trim_end().to_string();

        if line == "." {
            match vfs.write_file(filename, &builder, false) {
                Ok(_) => println!("Saved {}", filename),
                Err(e) => println!("\x1b[31mSave failed: {}\x1b[0m", e),
            }
            return;
        }
        if line == ":q" {
            println!("Exited without saving");
            return;
        }
        builder.push_str(&line);
        builder.push('\n');
    }
}
