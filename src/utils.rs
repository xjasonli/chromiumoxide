use std::path::{Path, PathBuf};
use crate::js::IntoJsArgs;

/// Write to file with configured runtime
pub(crate) async fn write<P: AsRef<Path> + Unpin, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> std::io::Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "async-std-runtime")] {
            async_std::fs::write(path.as_ref(), contents.as_ref()).await
        } else if #[cfg(feature = "tokio-runtime")] {
            tokio::fs::write(path.as_ref(), contents.as_ref()).await
        }
    }
}

/// Canonicalize path
///
/// Chromium sandboxing does not support Window UNC paths which are used by Rust
/// when the path is relative. See <https://bugs.chromium.org/p/chromium/issues/detail?id=1415018>.
pub(crate) async fn canonicalize<P: AsRef<Path> + Unpin>(path: P) -> std::io::Result<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "async-std-runtime")] {
            let path: PathBuf = async_std::fs::canonicalize(path.as_ref()).await?.into();
        } else if #[cfg(feature = "tokio-runtime")] {
            let path = tokio::fs::canonicalize(path.as_ref()).await?;
        }
    }
    Ok(dunce::simplified(&path).to_path_buf())
}

/// Absolute path
///
pub(crate) fn absolute(path: PathBuf) -> std::io::Result<PathBuf> {
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(dunce::simplified(&path).to_path_buf())
}

/// Canonicalize path except if target binary is snap, in this case only make the path absolute
///
pub(crate) async fn canonicalize_except_snap(path: PathBuf) -> std::io::Result<PathBuf> {
    // Canonalize paths to reduce issues with sandboxing
    let executable_cleaned: PathBuf = canonicalize(&path).await?;

    // Handle case where executable is provided by snap, ignore canonicalize result and only make path absolute
    Ok(if executable_cleaned.to_str().unwrap().ends_with("/snap") {
        absolute(path).unwrap()
    } else {
        executable_cleaned
    })
}

pub(crate) mod base64 {
    use base64::engine::general_purpose::STANDARD;
    use base64::{DecodeError, Engine};

    /// Decode base64 using the standard alphabet and padding
    pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(input)
    }
}

/// Creates a javascript function string as `(<function>)("<param 1>", "<param
/// 2>")`
pub fn evaluation_string<'a, Args: IntoJsArgs<'a>>(
    function: impl AsRef<str>,
    params: Args
) -> Result<String, serde_json::Error> {
    let values = params.into_vec();
    //let values = params.into_json_values()?;
    let params = values
        .iter()
        .map(|s| serde_json::to_string(s))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(format!("({})({})", function.as_ref(), params.join(", ")))
}

/// Tries to identify whether this a javascript function
pub fn is_likely_js_function(function: impl AsRef<str>) -> bool {
    let mut fun = function.as_ref().trim_start();
    if fun.is_empty() {
        return false;
    }
    let mut offset = 0;

    if fun.starts_with("async ") {
        offset = "async ".len() - 1
    }

    if fun[offset..].trim_start().starts_with("function ") {
        return true;
    } else if skip_args(&mut fun) {
        // attempt to detect arrow functions by stripping the leading arguments and
        // looking for the arrow
        if fun.trim_start().starts_with("=>") {
            return true;
        }
    }
    false
}

/// This attempts to strip any leading pair of parentheses from the input
///
/// `()=>` -> `=>`
/// `(abc, def)=>` -> `=>`
fn skip_args(input: &mut &str) -> bool {
    if !input.starts_with('(') {
        return false;
    }
    let mut open = 1;
    let mut closed = 0;
    *input = &input[1..];
    while !input.is_empty() && open != closed {
        if let Some(idx) = input.find(&['(', ')'] as &[_]) {
            if &input[idx..=idx] == ")" {
                closed += 1;
            } else {
                open += 1;
            }
            *input = &input[idx + 1..];
        } else {
            break;
        }
    }

    open == closed
}

/// JSON encoding fix utilities for handling malformed Unicode escapes in CDP messages
pub(crate) mod json_encoding {
    use encoding_rs::{Encoding, GBK, GB18030, BIG5};
    use once_cell::sync::Lazy;
    use fancy_regex::Regex;

    /// Matches lone UTF-16 surrogates (D800-DFFF) that are not part of a valid pair
    static LONE_SURROGATE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"\\u[dD][89a-fA-F][0-9a-fA-F]{2}(?!\\u[dD][c-fC-F][0-9a-fA-F]{2})|(?<!\\u[dD][89a-fA-F][0-9a-fA-F]{2})\\u[dD][c-fC-F][0-9a-fA-F]{2}"
        ).unwrap()
    });

    /// Matches sequences of Latin-1 high bytes (U+0080-U+00FF) that might be GBK-encoded
    /// These patterns indicate Chrome has treated GBK bytes as Latin-1
    static LATIN1_HIGH_BYTES: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?:\\u00[8-9a-fA-F][0-9a-fA-F]){2,}").unwrap()
    });

    /// Matches any Unicode escape sequence in the form \uXXXX
    static UNICODE_ESCAPE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap()
    });

    /// Attempts to decode a Unicode codepoint as a multi-byte character using the specified encoding
    /// 
    /// This is used to recover original text from malformed Unicode escapes that may have been
    /// created when non-UTF-8 encoded text (like GBK) was incorrectly converted to Unicode.
    /// 
    /// Returns `Some(String)` if the codepoint successfully decodes to a valid character,
    /// `None` otherwise.
    #[allow(dead_code)]
    fn try_decode_codepoint(cp: u16, enc: &'static Encoding) -> Option<String> {
        let bytes = [(cp >> 8) as u8, (cp & 0xFF) as u8];
        let (s, _, had_errors) = enc.decode(&bytes);
        
        // Only accept if decoding succeeded and result contains valid printable characters
        (!had_errors && s.chars().all(|c| !c.is_control() || "\t\n\r".contains(c)))
            .then(|| s.into_owned())
    }

    /// Attempts to decode an entire string value from JSON by treating Latin-1 Unicode escapes as GBK bytes
    /// 
    /// When Chrome encounters GBK-encoded bytes, it may represent them as Latin-1 Unicode escapes
    /// in the range U+0080-U+00FF (e.g., GBK byte 0xB2 becomes \u00B2).
    /// 
    /// This function collects ONLY Latin-1 range escapes and attempts GBK decoding.
    fn try_decode_json_string_value(json_substr: &str, enc: &'static Encoding) -> Option<String> {
        let mut bytes = Vec::new();
        let mut has_high_bytes = false;
        let mut all_latin1 = true;
        
        // Collect bytes from Latin-1 range Unicode escapes ONLY
        for cap in UNICODE_ESCAPE.captures_iter(json_substr) {
            if let Ok(cap) = cap {
                if let Some(hex_match) = cap.get(1) {
                    if let Ok(cp) = u16::from_str_radix(hex_match.as_str(), 16) {
                        if cp < 0x100 {
                            // Latin-1 range - treat as byte
                            let byte = cp as u8;
                            bytes.push(byte);
                            if byte >= 0x80 {
                                has_high_bytes = true;
                            }
                        } else {
                            // Non-Latin-1 Unicode found - this string has mixed encoding
                            all_latin1 = false;
                            break;
                        }
                    }
                }
            }
        }
        
        // Only decode if:
        // 1. All escapes are Latin-1 (no mixed Unicode)
        // 2. We found high bytes (0x80-0xFF, indicating potential GBK)
        if !all_latin1 || !has_high_bytes || bytes.is_empty() {
            return None;
        }
        
        tracing::debug!("Attempting to decode {} Latin-1 bytes: {:02X?}", bytes.len(), bytes);
        
        // Try to decode the byte sequence with the specified encoding
        let (decoded, _, had_errors) = enc.decode(&bytes);
        
        if had_errors {
            tracing::debug!("Decoding with {} had errors", enc.name());
            return None;
        }
        
        if decoded.is_empty() {
            return None;
        }
        
        // Verify the decoded string looks reasonable
        let valid_chars = decoded.chars().filter(|c| !c.is_control() || "\t\n\r".contains(*c)).count();
        if valid_chars == 0 {
            tracing::debug!("No valid characters after decoding");
            return None;
        }
        
        tracing::info!("Successfully decoded {} Latin-1 bytes -> '{}' using {}", 
                      bytes.len(), decoded, enc.name());
        Some(decoded.into_owned())
    }

    /// Fixes JSON encoding issues by attempting to recover malformed Unicode escapes
    /// 
    /// This function handles cases where Chrome DevTools Protocol returns JSON with:
    /// 1. Lone UTF-16 surrogates (codepoints in the range D800-DFFF that aren't part of a valid pair)
    /// 2. Latin-1 high bytes (U+0080-U+00FF) that represent GBK-encoded text
    /// 
    /// This typically happens when HTTP response headers contain non-UTF-8 encoded text
    /// (e.g., GBK-encoded Chinese filenames in Content-Disposition headers).
    /// 
    /// The function:
    /// 1. Detects problematic patterns using regex
    /// 2. Attempts to decode the entire problematic string using common multi-byte encodings
    /// 3. Falls back to replacing individual lone surrogates with U+FFFD if whole-string recovery fails
    /// 4. Preserves valid surrogate pairs (like emoji) unchanged
    /// 
    /// # Arguments
    /// * `json` - JSON string potentially containing malformed Unicode escapes
    /// 
    /// # Returns
    /// A corrected JSON string with encoding issues either recovered or replaced
    pub(crate) fn fix_json_encoding(json: &str) -> String {
        let has_lone_surrogates = LONE_SURROGATE.is_match(json).unwrap_or(false);
        let has_latin1_pattern = LATIN1_HIGH_BYTES.is_match(json).unwrap_or(false);
        
        // Fast path: return immediately if no encoding issues detected
        if !has_lone_surrogates && !has_latin1_pattern {
            return json.to_string();
        }
        
        if has_lone_surrogates {
            tracing::debug!("Detected lone surrogates in JSON, attempting encoding recovery");
        }
        if has_latin1_pattern {
            tracing::debug!("Detected Latin-1 high byte pattern in JSON, attempting encoding recovery");
        }
        
        // Try to recover Latin-1 encoded sequences first (most reliable)
        // Find all sequences of \u00XX (Latin-1 high bytes) and try to decode them
        let result = LATIN1_HIGH_BYTES.replace_all(json, |caps: &fancy_regex::Captures<'_>| {
            let matched = caps.get(0).unwrap().as_str();
            
            // Extract bytes from the matched Latin-1 escape sequence
            let mut bytes = Vec::new();
            for cap in UNICODE_ESCAPE.captures_iter(matched) {
                if let Ok(cap) = cap {
                    if let Some(hex_match) = cap.get(1) {
                        if let Ok(cp) = u16::from_str_radix(hex_match.as_str(), 16) {
                            if cp < 0x100 {
                                bytes.push(cp as u8);
                            }
                        }
                    }
                }
            }
            
            // Try to decode with common encodings
            for enc in [GBK, GB18030, BIG5] {
                let (decoded, _, had_errors) = enc.decode(&bytes);
                if !had_errors && !decoded.is_empty() {
                    // Verify it looks reasonable
                    let valid_chars = decoded.chars().filter(|c| !c.is_control() || "\t\n\r".contains(*c)).count();
                    if valid_chars > 0 && valid_chars * 2 >= decoded.chars().count() {
                        tracing::info!("Successfully decoded Latin-1 sequence ({} bytes) -> '{}' using {}", 
                                     bytes.len(), decoded, enc.name());
                        return decoded.into_owned();
                    }
                }
            }
            
            // If decoding failed, keep original
            matched.to_string()
        }).into_owned();
        
        tracing::debug!("Latin-1 recovery done, now handling lone surrogates");
        
        // Now handle lone surrogates (fallback to character replacement)
        UNICODE_ESCAPE.replace_all(&result, |caps: &fancy_regex::Captures<'_>| {
            let cp = u16::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap();
            
            // Only process codepoints in the surrogate range (D800-DFFF)
            if !(0xD800..=0xDFFF).contains(&cp) {
                // Not a surrogate, keep as-is
                return caps.get(0).unwrap().as_str().to_string();
            }
            
            // Extract the byte from the surrogate
            let byte = (cp & 0xFF) as u8;
            let byte_array = [byte];
            
            // Try to decode as single-byte character using various encodings
            for enc in [GBK, GB18030, BIG5] {
                let (decoded, _, had_errors) = enc.decode(&byte_array);
                if !had_errors && !decoded.is_empty() && !decoded.chars().all(|c| c.is_control()) {
                    tracing::warn!("Recovered U+{:04X} (byte 0x{:02X}) as {} -> {}", cp, byte, enc.name(), decoded);
                    return decoded.into_owned();
                }
            }
            
            // Fallback: use Unicode replacement character
            tracing::warn!("Cannot recover U+{:04X}, using replacement char", cp);
            "\u{FFFD}".to_string()
        }).into_owned()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_valid_emoji_unchanged() {
            // Valid surrogate pair (emoji) should not be modified
            let json = r#"{"emoji":"\uD83D\uDE00"}"#;  // 😀
            let fixed = fix_json_encoding(json);
            assert_eq!(json, fixed);
        }

        #[test]
        fn test_lone_surrogate_replaced() {
            // Lone trailing surrogate should be replaced
            let json = r#"{"text":"test\uDC63"}"#;
            let fixed = fix_json_encoding(json);
            
            // Should either be recovered (if it matches an encoding) or replaced with �
            assert_ne!(json, fixed);
            assert!(!fixed.contains(r"\uDC63"));
        }

        #[test]
        fn test_no_surrogates_unchanged() {
            // JSON without surrogates should be unchanged
            let json = r#"{"text":"hello world","num":123}"#;
            let fixed = fix_json_encoding(json);
            assert_eq!(json, fixed);
        }

        #[test]
        fn test_valid_json_parseable() {
            // Fixed JSON should be parseable
            let json = r#"{"field":"\uDC63"}"#;
            let fixed = fix_json_encoding(json);
            
            let result: Result<serde_json::Value, _> = serde_json::from_str(&fixed);
            assert!(result.is_ok(), "Fixed JSON should be parseable");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_js_function() {
        assert!(is_likely_js_function("function abc() {}"));
        assert!(is_likely_js_function("async function abc() {}"));
        assert!(is_likely_js_function("() => {}"));
        assert!(is_likely_js_function("(abc, def) => {}"));
        assert!(is_likely_js_function("((abc), (def)) => {}"));
        assert!(is_likely_js_function("() => Promise.resolve(100 / 25)"));
    }
}
