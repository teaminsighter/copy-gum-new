// Content Type Detection Module
// Automatically detects the type of clipboard content

use regex::Regex;
use std::sync::OnceLock;

/// Content type identifier
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Color,      // Hex color codes (#RRGGBB, #RGB)
    Url,        // URLs (http://, https://, www.)
    Email,      // Email addresses
    Phone,      // Phone numbers
    Number,     // Pure numbers
    Code,       // Code snippets
    Text,       // Default fallback
}

impl ContentType {
    /// Convert ContentType enum to string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Color => "color",
            ContentType::Url => "links",
            ContentType::Email => "email",
            ContentType::Phone => "phone",
            ContentType::Number => "number",
            ContentType::Code => "code",
            ContentType::Text => "text",
        }
    }
}

// Lazy-initialized regex patterns (compiled once, reused many times)
static HEX_COLOR_REGEX: OnceLock<Regex> = OnceLock::new();
static URL_REGEX: OnceLock<Regex> = OnceLock::new();
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();
static NUMBER_REGEX: OnceLock<Regex> = OnceLock::new();
static CODE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Initialize all regex patterns
fn init_regexes() {
    // Hex color: #RGB or #RRGGBB (with optional alpha)
    HEX_COLOR_REGEX.get_or_init(|| {
        Regex::new(r"^#([0-9A-Fa-f]{3}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$").unwrap()
    });

    // URL: http://, https://, www., or common TLDs
    URL_REGEX.get_or_init(|| {
        Regex::new(r"^(https?://|www\.)[^\s]+|^[^\s]+\.(com|org|net|edu|gov|io|co|app|dev|tech|ai|me|info|biz)(/[^\s]*)?$").unwrap()
    });

    // Email: basic email pattern
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });

    // Phone: various formats (with optional country code, spaces, dashes, parentheses)
    PHONE_REGEX.get_or_init(|| {
        Regex::new(r"^(\+?1?\s?)?(\([0-9]{3}\)|[0-9]{3})[\s\-]?[0-9]{3}[\s\-]?[0-9]{4}$").unwrap()
    });

    // Number: pure digits with optional decimal point and sign
    NUMBER_REGEX.get_or_init(|| {
        Regex::new(r"^[+-]?(\d+\.?\d*|\.\d+)$").unwrap()
    });

    // Code: contains common programming patterns
    CODE_REGEX.get_or_init(|| {
        Regex::new(r"(function\s+\w+|const\s+\w+\s*=|let\s+\w+\s*=|var\s+\w+\s*=|class\s+\w+|def\s+\w+|import\s+|export\s+|return\s+|public\s+|private\s+|protected\s+|\{[^}]*\}|;$|\=\>|\:\:)").unwrap()
    });
}

/// Detect the content type of the given text
///
/// Detection order (priority):
/// 1. Hex Color (#RRGGBB)
/// 2. URL (http://, https://, www.)
/// 3. Email (contains @)
/// 4. Phone (phone number patterns)
/// 5. Number (pure digits)
/// 6. Code (programming patterns)
/// 7. Text (default fallback)
pub fn detect_content_type(content: &str) -> ContentType {
    // Initialize regex patterns (only happens once)
    init_regexes();

    // Trim whitespace for accurate detection
    let trimmed = content.trim();

    // Empty content defaults to text
    if trimmed.is_empty() {
        return ContentType::Text;
    }

    // 1. Check for hex color (highest priority - most specific)
    if let Some(regex) = HEX_COLOR_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Color;
        }
    }

    // 2. Check for URL
    if let Some(regex) = URL_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Url;
        }
    }

    // 3. Check for email
    if let Some(regex) = EMAIL_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Email;
        }
    }

    // 4. Check for phone number
    if let Some(regex) = PHONE_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Phone;
        }
    }

    // 5. Check for number (after phone to avoid false positives)
    if let Some(regex) = NUMBER_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Number;
        }
    }

    // 6. Check for code (multiline or contains programming keywords)
    if let Some(regex) = CODE_REGEX.get() {
        if regex.is_match(trimmed) {
            return ContentType::Code;
        }
    }

    // 7. Default to text
    ContentType::Text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_detection() {
        // Valid hex colors
        assert_eq!(detect_content_type("#FFF"), ContentType::Color);
        assert_eq!(detect_content_type("#FFFFFF"), ContentType::Color);
        assert_eq!(detect_content_type("#FF5733"), ContentType::Color);
        assert_eq!(detect_content_type("#ff5733"), ContentType::Color);
        assert_eq!(detect_content_type("#FF5733AA"), ContentType::Color); // With alpha

        // Invalid hex colors
        assert_ne!(detect_content_type("#GGG"), ContentType::Color);
        assert_ne!(detect_content_type("FF5733"), ContentType::Color); // Missing #
        assert_ne!(detect_content_type("#FF57"), ContentType::Color); // Wrong length
    }

    #[test]
    fn test_url_detection() {
        // Valid URLs
        assert_eq!(detect_content_type("https://example.com"), ContentType::Url);
        assert_eq!(detect_content_type("http://example.com"), ContentType::Url);
        assert_eq!(detect_content_type("www.example.com"), ContentType::Url);
        assert_eq!(detect_content_type("example.com"), ContentType::Url);
        assert_eq!(detect_content_type("example.io/path"), ContentType::Url);
        assert_eq!(detect_content_type("api.example.co"), ContentType::Url);

        // Invalid URLs
        assert_ne!(detect_content_type("not a url"), ContentType::Url);
        assert_ne!(detect_content_type("example"), ContentType::Url);
    }

    #[test]
    fn test_email_detection() {
        // Valid emails
        assert_eq!(detect_content_type("user@example.com"), ContentType::Email);
        assert_eq!(detect_content_type("test.user@example.co.uk"), ContentType::Email);
        assert_eq!(detect_content_type("name+tag@domain.org"), ContentType::Email);

        // Invalid emails
        assert_ne!(detect_content_type("not an email"), ContentType::Email);
        assert_ne!(detect_content_type("@example.com"), ContentType::Email);
        assert_ne!(detect_content_type("user@"), ContentType::Email);
    }

    #[test]
    fn test_phone_detection() {
        // Valid phone numbers
        assert_eq!(detect_content_type("555-123-4567"), ContentType::Phone);
        assert_eq!(detect_content_type("(555) 123-4567"), ContentType::Phone);
        assert_eq!(detect_content_type("5551234567"), ContentType::Phone);
        assert_eq!(detect_content_type("+1 555 123 4567"), ContentType::Phone);

        // Invalid phone numbers
        assert_ne!(detect_content_type("123"), ContentType::Phone);
        assert_ne!(detect_content_type("not a phone"), ContentType::Phone);
    }

    #[test]
    fn test_number_detection() {
        // Valid numbers
        assert_eq!(detect_content_type("123"), ContentType::Number);
        assert_eq!(detect_content_type("123.456"), ContentType::Number);
        assert_eq!(detect_content_type("-123"), ContentType::Number);
        assert_eq!(detect_content_type("+456.78"), ContentType::Number);
        assert_eq!(detect_content_type(".5"), ContentType::Number);

        // Invalid numbers
        assert_ne!(detect_content_type("123abc"), ContentType::Number);
        assert_ne!(detect_content_type("not a number"), ContentType::Number);
    }

    #[test]
    fn test_code_detection() {
        // Valid code patterns
        assert_eq!(detect_content_type("function test() { return 42; }"), ContentType::Code);
        assert_eq!(detect_content_type("const value = 123;"), ContentType::Code);
        assert_eq!(detect_content_type("let x = 10;"), ContentType::Code);
        assert_eq!(detect_content_type("class MyClass {}"), ContentType::Code);
        assert_eq!(detect_content_type("def my_function():"), ContentType::Code);
        assert_eq!(detect_content_type("import React from 'react';"), ContentType::Code);
        assert_eq!(detect_content_type("public static void main()"), ContentType::Code);

        // Not code
        assert_ne!(detect_content_type("This is plain text"), ContentType::Code);
    }

    #[test]
    fn test_text_default() {
        // Everything else defaults to text
        assert_eq!(detect_content_type("Hello World"), ContentType::Text);
        assert_eq!(detect_content_type("Some random text"), ContentType::Text);
        assert_eq!(detect_content_type("Lorem ipsum dolor sit amet"), ContentType::Text);
        assert_eq!(detect_content_type(""), ContentType::Text);
    }

    #[test]
    fn test_priority_order() {
        // Hex color has highest priority
        assert_eq!(detect_content_type("#123"), ContentType::Color);

        // URL before email before phone before number
        // (This is ensured by the detection order in detect_content_type)
    }
}
