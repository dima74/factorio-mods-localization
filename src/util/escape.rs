use std::sync::LazyLock;

use regex::{Captures, Regex};

/// .ini file looks like this:
/// ```ini
/// [section1]
/// key1=value1
/// key2=value2
/// ```
/// We need to escape (wrap in quotes) values which contains quotes or semicolon, because:
/// - semicolon is used for commenting, so by default everything after semicolon is ignored
/// - quotes are used for escaping, so by default something strange happens
pub fn escape_strings_in_ini_file(ini_content: &str) -> String {
    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?mR)^([^\[=\n]*)=([^\n]*)$").unwrap());
    REGEX.replace_all(ini_content, |captures: &Captures| {
        let key = &captures[1];
        let value = &captures[2];
        let should_escape = value.contains('"') || value.contains(';');
        let already_escaped = value.starts_with('"') && value.ends_with('"');
        if should_escape && !already_escaped {
            format!("{}=\"{}\"", key, value)
        } else {
            captures[0].to_owned()
        }
    }).into_owned()
}

#[cfg(test)]
mod tests {
    use super::escape_strings_in_ini_file;

    fn test(source: &str, expected: &str) {
        assert_eq!(expected, escape_strings_in_ini_file(source));
    }

    #[test]
    fn test_no_escape() {
        test("[section]", "[section]");
        test("key=value", "key=value");
        test(r#"key="value""#, r#"key="value""#);
        test(r"
[section]
key1=value1
key2=value2
        ", r"
[section]
key1=value1
key2=value2
        ");
    }

    #[test]
    fn test_escape() {
        test(r#"key=foo"bar"#, r#"key="foo"bar""#);
        test(r#"key=foo;bar"#, r#"key="foo;bar""#);
        test(r#"
key1=value;1
key2=value;2
        "#, r#"
key1="value;1"
key2="value;2"
        "#);
        test(r#"
[se"ct;ion]
key1="value1
key2=value2"
key2=v;a"l;u"e;3
        "#, r#"
[se"ct;ion]
key1=""value1"
key2="value2""
key2="v;a"l;u"e;3"
        "#);
    }

    #[test]
    fn test_escape_crlf() {
        test(
            "key1=foo\"bar\r\nkey2=value2",
            "key1=\"foo\"bar\"\r\nkey2=value2"
        );
    }
}
