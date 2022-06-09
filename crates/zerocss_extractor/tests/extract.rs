use zerocss_extractor::extract;

use std::collections::HashSet;

#[test]
fn extract_html() {
    assert_eq!(
        extract("<div class=\"foo-3 bar-3 baz-eggs-100\"></div>"),
        vec!["bar-3", "baz-eggs-100", "foo-3"]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
    );
}

#[test]
fn extract_slash() {
    assert_eq!(
        extract("\"foo-3 bar-3 baz-eggs-100 slash/ing\""),
        vec!["foo-3", "bar-3", "baz-eggs-100", "slash", "ing"]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
    );
}

#[test]
fn extract_invalid() {
    assert_eq!(
        extract("\"foo-3 bar-3 baz-eggs-100 π\""),
        vec!["foo-3", "bar-3", "baz-eggs-100"]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
    );
}
