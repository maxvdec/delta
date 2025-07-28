use cfont::font::load;

#[test]
fn test_load_font() {
    load::load_system_font("Arial").unwrap();
}

#[test]
fn should_fail() {
    let result = load::load_font("ThisFontDoesNotExist");
    assert!(
        result.is_err(),
        "Expected an error when loading a non-existent font: {}",
        result.unwrap_err()
    );
}
