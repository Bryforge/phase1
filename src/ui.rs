use crate::registry;

const BOX_WIDTH: usize = 28;

pub fn print_boot(version: &str) {
    border_top();
    box_line("phase1 operator console");
    box_line(&format!("version {}", version));
    box_line("mode mobile / sandbox");
    border_mid("subsystems");
    box_line("vfs.proc  sched.jobs");
    box_line("net.pcie  mem.cr3");
    box_line("py.c      browser.plug");
    border_mid("quickstart");
    box_line("help | man browser | ps");
    box_line("ls / | plugins | exit");
    border_bottom();
    println!("[ok] boot nominal");
    println!("[tip] man browser | browser phase1");
    println!();
}

pub fn print_help() {
    print!("{}", registry::command_map());
}

fn border_top() {
    println!("+----------------------------+");
}

fn border_bottom() {
    println!("+----------------------------+");
}

fn border_mid(label: &str) {
    let title = format!(" {} ", label);
    let used = title.len().min(BOX_WIDTH);
    let fill = BOX_WIDTH.saturating_sub(used);
    println!("+{}{}+", title, "-".repeat(fill));
}

fn box_line(text: &str) {
    let clipped: String = text.chars().take(BOX_WIDTH).collect();
    println!("| {:<width$} |", clipped, width = BOX_WIDTH);
}

#[cfg(test)]
mod tests {
    use super::BOX_WIDTH;

    #[test]
    fn mobile_box_width_stays_small() {
        assert!(BOX_WIDTH <= 32);
    }
}
