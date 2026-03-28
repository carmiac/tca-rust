//! Parser for the flat key: "value" YAML subset used by Base16/24 scheme files.

use std::collections::HashMap;
use std::io::{self, BufRead};

use crate::HexColorError;

/// A parsed base24 scheme file represented as a flat key→value map.
///
/// Keys are lowercased. Values are raw strings (may be quoted in the source).
pub type RawSlots = HashMap<String, String>;

/// Parse the flat `key: "value"` YAML subset used by Base16/24 scheme files.
///
/// - Blank lines and lines starting with `#` are skipped.
/// - Inline comments outside of quoted strings are stripped.
/// - Surrounding quotes (single or double) are removed from values.
/// - Keys are lowercased for case-insensitive lookup.
pub fn parse_base24(reader: impl io::Read) -> io::Result<RawSlots> {
    let mut map = HashMap::new();
    for line in io::BufReader::new(reader).lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            // Lines without a colon are silently skipped (e.g. "---" document markers).
            continue;
        };
        let key = key.trim().to_lowercase();
        let value = strip_inline_comment(value.trim()).trim().to_string();
        let value = value.trim_matches('"').trim_matches('\'').to_string();
        map.insert(key, value);
    }
    Ok(map)
}

/// Strip a trailing inline comment (`# …`) that appears outside of quotes.
fn strip_inline_comment(s: &str) -> &str {
    let mut in_quote: Option<char> = None;
    for (i, c) in s.char_indices() {
        match (in_quote, c) {
            (None, '"') | (None, '\'') => in_quote = Some(c),
            (Some(q), c) if c == q => in_quote = None,
            (None, '#') => return &s[..i],
            _ => {}
        }
    }
    s
}

/// Normalize a raw hex string to `#rrggbb` (lowercase, with leading `#`).
///
/// Accepts values with or without a leading `#`.
pub fn normalize_hex(s: &str) -> Result<String, HexColorError> {
    let hex = s.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(HexColorError::InvalidLength(hex.len()));
    }
    // Validate all characters are hex digits by attempting a parse.
    u32::from_str_radix(hex, 16).map_err(HexColorError::InvalidHex)?;
    Ok(format!("#{}", hex.to_lowercase()))
}

/// Returns `true` when the theme is dark.
///
/// Dark themes have a darker background (base00) than their brightest foreground
/// (base07). Compares the sum of RGB components of each.
pub fn is_dark(base00: &str, base07: &str) -> bool {
    fn brightness(hex: &str) -> u32 {
        let Ok((r, g, b)) = crate::hex_to_rgb(hex) else {
            return 0;
        };
        r as u32 + g as u32 + b as u32
    }
    brightness(base00) < brightness(base07)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_key_value() {
        let input = b"scheme: \"My Theme\"\nauthor: \"Ada\"\nbase00: \"1e1e2e\"\n";
        let slots = parse_base24(&input[..]).unwrap();
        assert_eq!(slots.get("scheme").map(String::as_str), Some("My Theme"));
        assert_eq!(slots.get("author").map(String::as_str), Some("Ada"));
        assert_eq!(slots.get("base00").map(String::as_str), Some("1e1e2e"));
    }

    #[test]
    fn parse_skips_blank_lines_and_comments() {
        let input = b"# comment\n\nscheme: \"X\"\n";
        let slots = parse_base24(&input[..]).unwrap();
        assert_eq!(slots.len(), 1);
    }

    #[test]
    fn parse_strips_inline_comment() {
        let input = b"base00: \"aabbcc\" # background\n";
        let slots = parse_base24(&input[..]).unwrap();
        assert_eq!(slots.get("base00").map(String::as_str), Some("aabbcc"));
    }

    #[test]
    fn parse_keys_are_lowercased() {
        let input = b"BASE00: \"aabbcc\"\n";
        let slots = parse_base24(&input[..]).unwrap();
        assert!(slots.contains_key("base00"));
    }

    #[test]
    fn parse_document_separator_is_skipped() {
        let input = b"---\nscheme: \"Y\"\n";
        let slots = parse_base24(&input[..]).unwrap();
        assert_eq!(slots.len(), 1);
    }

    #[test]
    fn normalize_hex_with_hash() {
        assert_eq!(normalize_hex("#FF5533").unwrap(), "#ff5533");
    }

    #[test]
    fn normalize_hex_without_hash() {
        assert_eq!(normalize_hex("FF5533").unwrap(), "#ff5533");
    }

    #[test]
    fn normalize_hex_wrong_length() {
        assert!(normalize_hex("fff").is_err());
        assert!(normalize_hex("ff553300").is_err());
    }

    #[test]
    fn is_dark_dark_theme() {
        assert!(is_dark("#1e1e2e", "#cdd6f4")); // dark bg, light fg
    }

    #[test]
    fn is_dark_light_theme() {
        assert!(!is_dark("#fdf6e3", "#657b83")); // light bg, dark fg
    }
}
