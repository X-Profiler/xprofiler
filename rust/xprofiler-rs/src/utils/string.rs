//! String utilities for xprofiler

use std::collections::HashMap;
use std::fmt::Write;

/// Truncate a string to a maximum length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Truncate a string to a maximum length with custom suffix
pub fn truncate_with_suffix(s: &str, max_len: usize, suffix: &str) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= suffix.len() {
        suffix.to_string()
    } else {
        format!("{}{}", &s[..max_len - suffix.len()], suffix)
    }
}

/// Pad a string to a specific width
pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        let padding = width - s.len();
        format!("{}{}", pad_char.to_string().repeat(padding), s)
    }
}

/// Pad a string to a specific width on the right
pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        let padding = width - s.len();
        format!("{}{}", s, pad_char.to_string().repeat(padding))
    }
}

/// Center a string within a specific width
pub fn center(s: &str, width: usize, pad_char: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        let total_padding = width - s.len();
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        
        format!(
            "{}{}{}",
            pad_char.to_string().repeat(left_padding),
            s,
            pad_char.to_string().repeat(right_padding)
        )
    }
}

/// Convert string to snake_case
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    let mut prev_was_digit = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && (!prev_was_upper || (i + 1 < s.len() && s.chars().nth(i + 1).map_or(false, |next| next.is_lowercase()))) {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
            prev_was_upper = true;
            prev_was_digit = false;
        } else if c.is_digit(10) {
            if i > 0 && !prev_was_digit {
                result.push('_');
            }
            result.push(c);
            prev_was_upper = false;
            prev_was_digit = true;
        } else if c.is_alphanumeric() {
            result.push(c);
            prev_was_upper = false;
            prev_was_digit = false;
        } else {
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            prev_was_upper = false;
            prev_was_digit = false;
        }
    }
    
    result.trim_end_matches('_').to_string()
}

/// Convert string to camelCase
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c.to_lowercase().next().unwrap_or(c));
            }
        } else {
            capitalize_next = true;
        }
    }
    
    result
}

/// Convert string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    let camel = to_camel_case(s);
    if camel.is_empty() {
        return camel;
    }
    
    let mut chars = camel.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert string to kebab-case
pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

/// Escape special characters for JSON
pub fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{08}' => result.push_str("\\b"),
            '\u{0C}' => result.push_str("\\f"),
            c if c.is_control() => {
                write!(result, "\\u{:04x}", c as u32).unwrap();
            }
            c => result.push(c),
        }
    }
    
    result
}

/// Unescape JSON string
pub fn unescape_json(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('b') => result.push('\u{08}'),
                Some('f') => result.push('\u{0C}'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if hex.len() != 4 {
                        return Err("Invalid unicode escape sequence".to_string());
                    }
                    match u32::from_str_radix(&hex, 16) {
                        Ok(code) => {
                            if let Some(unicode_char) = char::from_u32(code) {
                                result.push(unicode_char);
                            } else {
                                return Err(format!("Invalid unicode code point: {}", code));
                            }
                        }
                        Err(_) => return Err(format!("Invalid hex in unicode escape: {}", hex)),
                    }
                }
                Some(c) => return Err(format!("Invalid escape sequence: \\{}", c)),
                None => return Err("Unexpected end of string after backslash".to_string()),
            }
        } else {
            result.push(c);
        }
    }
    
    Ok(result)
}

/// Split string by delimiter and trim whitespace
pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter)
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

/// Join strings with a delimiter
pub fn join_with_delimiter<T: AsRef<str>>(items: &[T], delimiter: &str) -> String {
    items
        .iter()
        .map(|item| item.as_ref())
        .collect::<Vec<_>>()
        .join(delimiter)
}

/// Remove all whitespace from a string
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Normalize whitespace (replace multiple whitespace with single space)
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Count occurrences of a substring
pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    
    let mut count = 0;
    let mut start = 0;
    
    while let Some(pos) = haystack[start..].find(needle) {
        count += 1;
        start += pos + needle.len();
    }
    
    count
}

/// Replace multiple patterns with their replacements
pub fn replace_multiple(s: &str, replacements: &HashMap<&str, &str>) -> String {
    let mut result = s.to_string();
    
    for (pattern, replacement) in replacements {
        result = result.replace(pattern, replacement);
    }
    
    result
}

/// Check if string contains only ASCII characters
pub fn is_ascii_only(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii())
}

/// Check if string is a valid identifier (starts with letter/underscore, contains only alphanumeric/underscore)
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Generate a random string of specified length
pub fn random_string(length: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let mut seed = hasher.finish();
    
    let mut result = String::with_capacity(length);
    
    for _ in 0..length {
        // Simple LCG for pseudo-random generation
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (seed as usize) % CHARS.len();
        result.push(CHARS[index] as char);
    }
    
    result
}

/// Calculate Levenshtein distance between two strings
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    // Initialize first row and column
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1       // insertion
                ),
                matrix[i - 1][j - 1] + cost // substitution
            );
        }
    }
    
    matrix[len1][len2]
}

/// String similarity based on Levenshtein distance (0.0 to 1.0)
pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    let max_len = std::cmp::max(s1.chars().count(), s2.chars().count());
    if max_len == 0 {
        return 1.0;
    }
    
    let distance = levenshtein_distance(s1, s2);
    1.0 - (distance as f64 / max_len as f64)
}

/// Find the most similar string from a list
pub fn find_most_similar<'a>(target: &str, candidates: &[&'a str]) -> Option<(&'a str, f64)> {
    candidates
        .iter()
        .map(|&candidate| (candidate, string_similarity(target, candidate)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
}

/// Encode string to base64
pub fn encode_base64(input: &str) -> String {
    base64_encode(input.as_bytes())
}

/// Decode base64 string
pub fn decode_base64(input: &str) -> Result<String, String> {
    match base64_decode(input) {
        Ok(bytes) => String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e)),
        Err(e) => Err(e),
    }
}

/// Simple base64 encoding implementation
fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((b >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARS[(b & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Simple base64 decoding implementation
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let input = input.trim_end_matches('=');
    let mut result = Vec::new();
    
    for chunk in input.as_bytes().chunks(4) {
        let mut buf = [0u8; 4];
        
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = match byte {
                b'A'..=b'Z' => byte - b'A',
                b'a'..=b'z' => byte - b'a' + 26,
                b'0'..=b'9' => byte - b'0' + 52,
                b'+' => 62,
                b'/' => 63,
                _ => return Err(format!("Invalid base64 character: {}", byte as char)),
            };
        }
        
        let b = ((buf[0] as u32) << 18) | ((buf[1] as u32) << 12) | ((buf[2] as u32) << 6) | (buf[3] as u32);
        
        result.push((b >> 16) as u8);
        if chunk.len() > 2 {
            result.push((b >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(b as u8);
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 5), "he...");
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 3), "...");
        assert_eq!(truncate("hello", 2), "...");
    }

    #[test]
    fn test_padding() {
        assert_eq!(pad_left("hello", 10, ' '), "     hello");
        assert_eq!(pad_right("hello", 10, ' '), "hello     ");
        assert_eq!(center("hello", 11, ' '), "   hello   ");
        assert_eq!(center("hello", 10, ' '), "  hello   ");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("XMLHttpRequest"), "xml_http_request");
        assert_eq!(to_snake_case("iPhone5S"), "i_phone_5_s");
        
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("xml-http-request"), "xmlHttpRequest");
        
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("xml-http-request"), "XmlHttpRequest");
        
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
    }

    #[test]
    fn test_json_escape() {
        assert_eq!(escape_json("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_json("say \"hello\""), "say \\\"hello\\\"");
        
        assert_eq!(unescape_json("hello\\nworld").unwrap(), "hello\nworld");
        assert_eq!(unescape_json("say \\\"hello\\\"").unwrap(), "say \"hello\"");
    }

    #[test]
    fn test_string_utilities() {
        assert_eq!(split_and_trim("a, b , c ", ','), vec!["a", "b", "c"]);
        assert_eq!(join_with_delimiter(&["a", "b", "c"], ", "), "a, b, c");
        assert_eq!(remove_whitespace("a b c"), "abc");
        assert_eq!(normalize_whitespace("a   b\n\tc"), "a b c");
        assert_eq!(count_occurrences("hello hello world", "hello"), 2);
    }

    #[test]
    fn test_validation() {
        assert!(is_ascii_only("hello123"));
        assert!(!is_ascii_only("hello世界"));
        
        assert!(is_valid_identifier("hello_world"));
        assert!(is_valid_identifier("_private"));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier("hello-world"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("", "hello"), 5);
        assert_eq!(levenshtein_distance("hello", ""), 5);
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("hello", "hello"), 1.0);
        assert!(string_similarity("hello", "hallo") > 0.5);
        assert!(string_similarity("hello", "world") < 0.5);
    }

    #[test]
    fn test_find_most_similar() {
        let candidates = ["hello", "world", "hallo", "help"];
        let (best, score) = find_most_similar("helo", &candidates).unwrap();
        assert_eq!(best, "hello");
        assert!(score > 0.5);
    }

    #[test]
    fn test_base64() {
        let original = "Hello, World!";
        let encoded = encode_base64(original);
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(original, decoded);
        
        assert_eq!(encode_base64("Man"), "TWFu");
        assert_eq!(decode_base64("TWFu").unwrap(), "Man");
    }

    #[test]
    fn test_random_string() {
        let s1 = random_string(10);
        let s2 = random_string(10);
        
        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        assert_ne!(s1, s2); // Very unlikely to be the same
        assert!(s1.chars().all(|c| c.is_alphanumeric()));
    }
}