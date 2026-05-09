use std::fs;

#[test]
fn boot_screen_uses_japanese_hacker_greeting() {
    let boot = fs::read_to_string("src/boot_ui_static.rs").expect("boot ui source exists");
    assert!(boot.contains("こんにちは、ハッカー！"));
    assert!(boot.contains("boot_greeting_line(config, boot_stamp)"));
    assert!(!boot.contains("node TOKYO-01 | boot"));
}
