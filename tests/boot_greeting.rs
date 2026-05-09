use std::fs;

#[test]
fn boot_screen_uses_japanese_hacker_greeting() {
    let boot = fs::read_to_string("src/boot_ui_static.rs").expect("boot ui source exists");

    assert!(boot.contains("こんにちは、ハッカー！"));
    assert!(boot.contains("boot_greeting_line(config)"));
    assert!(boot.contains("boot_time_line(config, boot_stamp)"));
    assert!(boot.contains("let inner = width;"));
    assert!(boot.contains("fit_cell_text(text, inner)"));
    assert!(boot.contains("visible_cell_width(&fitted)"));
    assert!(boot.contains("ui device {} :: kern/vfs/proc"));

    assert!(!boot.contains("width.saturating_sub(2)"));
    assert!(!boot.contains("boot_greeting_line(config, boot_stamp)"));
    assert!(!boot.contains("greeting}{RESET} | boot"));
    assert!(!boot.contains("greeting} | boot"));
    assert!(!boot.contains("node TOKYO-01 | boot"));
}
