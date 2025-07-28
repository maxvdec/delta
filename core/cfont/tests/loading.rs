use cfont::font::load::get_system_font;

#[test]
fn load_font() {
    assert!(get_system_font("Arial").is_ok());
}

#[test]
fn has_core_font() {
    let font = get_system_font("Arial").unwrap();
    assert!(font.core_font.is_some());
}
