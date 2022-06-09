use lazy_static::lazy_static;

use regex::Regex;
use std::collections::HashSet;

/// Extracts all the possible utility classes from a string (as long as they are inside single or double quotes).
///
/// This works on almost every single templating language, but it's not perfect.
///
/// ## Arguments
///
/// * `code` - The code to search through.
///
/// ## Examples
///
/// ```
/// use zerocss_extractor::extract;
/// extract("<div class=\"foo-3 bar-3 baz-eggs-100\"></div>"); // ["bar-3", "baz-eggs-100", "foo-3"]
///
/// // It can also handle non-html code.
/// extract("\"foo-3 bar-3 baz-eggs-100 not/a/class\""); // ["foo-3", "bar-3", "baz-eggs-100"]
/// ```
pub fn extract(code: &str) -> HashSet<&str> {
    lazy_static! {
        static ref QUOTE_REGEX: Regex = Regex::new(r#"[`"']([^`"']+)[`"']"#).unwrap();
        static ref CLASS_REGEX: Regex = Regex::new(r"-{0,2}[a-z0-9-]+").unwrap();
    }

    let mut set = HashSet::new();

    for mat in QUOTE_REGEX.find_iter(code) {
        for cmat in CLASS_REGEX.find_iter(mat.as_str()) {
            set.insert(cmat.as_str());
        }
    }

    set
}
