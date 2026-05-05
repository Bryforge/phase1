pub fn print_boot(version: &str) {
    println!("+--[ phase1 // v{} ]------+");
    println!("| terminal os simulator      |");
    println!("| status: online / sandboxed |");
    println!("+--[ subsystems ]------------+");
    println!("| vfs.proc   sched.jobs      |");
    println!("| net.pcie   mem.cr3         |");
    println!("| py.c       browser.plugins |");
    println!("+--[ quickstart ]------------+");
    println!("| help | man browser | ps    |");
    println!("| ls / | plugins    | exit   |");
    println!("+----------------------------+");
    println!("[ok] boot nominal");
    println!("[tip] man browser | browser phase1");
    println!();
}

pub fn print_help() {
    println!("phase1 // command map");
    println!();
    println!("fs    : ls cd pwd cat mkdir touch rm cp mv tree echo");
    println!("proc  : ps top spawn jobs fg bg kill nice");
    println!("net   : ifconfig iwconfig wifi-scan wifi-connect ping nmcli");
    println!("host  : browser python gcc plugins ned");
    println!("arch  : lspci pcie cr3 loadcr3 cr4 pcide");
    println!("sys   : free df dmesg vmstat uname date uptime hostname");
    println!("user  : env export unset whoami id su history");
    println!("misc  : help man clear version sandbox exit");
    println!();
    println!("quick : man browser | browser phase1 | ps | ls /");
}
