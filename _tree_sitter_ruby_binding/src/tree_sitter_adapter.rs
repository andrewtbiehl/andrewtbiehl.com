// A work-in-progress adapter to the Tree-sitter highlight library.

// Escapes HTML text content.
//
// Not intended for use on other HTML content, such as attribute content.
fn escape_text_html(text: &str) -> String {
    let mut escaped_text = String::new();
    for c in text.chars() {
        match c {
            '&' => escaped_text.push_str("&amp;"),
            '<' => escaped_text.push_str("&lt;"),
            '>' => escaped_text.push_str("&gt;"),
            _ => escaped_text.push(c),
        }
    }
    escaped_text
}

// Convenience function for not highlighting code.
//
// HTML escapes the given text without highlighting it. Useful for when the code's
// language is not known.
pub fn no_highlight(code: &str) -> String {
    escape_text_html(code)
}
