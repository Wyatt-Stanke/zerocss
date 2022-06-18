#[allow(non_snake_case)]
use zerocss_css_ast::{BaseDeclaration, _css};

#[test]
fn test_base_declaration() {
    let base_declaration = BaseDeclaration::new("color", "red");
    assert_eq!(base_declaration.name, "color");
    assert_eq!(base_declaration.value, "red");
}

#[test]
fn test_underscore() {
    let base_declaration = _css("color", "red");
    assert_eq!(base_declaration.name, "color");
    assert_eq!(base_declaration.value, "red");
}