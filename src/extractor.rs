use std::sync::LazyLock;

use regex::Regex;

use crate::types::FilterMode;

// Rust note: `LazyLock` is like a global that's initialized once on first use.
// Compiling a Regex is expensive, so we do it once and reuse it.

/// Matches `**...**` — bold markers with at least one character inside.
static BOLD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*(.+?)\*\*").expect("bold regex must compile"));

/// Matches `==...==` — highlight markers with at least one character inside.
static HIGHLIGHT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"==(.+?)==").expect("highlight regex must compile"));

/// Extract bold and/or highlighted spans from a single line of text.
///
/// Returns `Some(extracted_text)` if any matching spans were found,
/// or `None` if the line has no marked text.
///
/// # Arguments
///
/// * `line` - The source line to extract from
/// * `mode` - Which markers to look for (bold, highlight, or both)
/// * `strip` - If `true`, remove the `**` and `==` delimiters from output
///
/// # Returns
///
/// `Some(String)` with the extracted spans joined by spaces, or `None`.
pub fn extract_marked_text(line: &str, mode: FilterMode, strip: bool) -> Option<String> {
    let mut spans: Vec<(usize, &str)> = Vec::new();

    if matches!(mode, FilterMode::Both | FilterMode::BoldOnly) {
        for m in BOLD_RE.find_iter(line) {
            spans.push((m.start(), m.as_str()));
        }
    }

    if matches!(mode, FilterMode::Both | FilterMode::HighlightOnly) {
        for m in HIGHLIGHT_RE.find_iter(line) {
            spans.push((m.start(), m.as_str()));
        }
    }

    if spans.is_empty() {
        return None;
    }

    // Sort by position so spans appear in document order
    spans.sort_by_key(|(pos, _)| *pos);

    // Deduplicate overlapping spans (e.g. bold containing highlight)
    spans.dedup_by(|a, b| {
        let b_end = b.0 + b.1.len();
        let a_end = a.0 + a.1.len();
        if a.0 <= b.0 && a_end >= b_end {
            // a contains b — drop b
            true
        } else if b.0 <= a.0 && b_end >= a_end {
            // b contains a — keep b, replace a
            *a = *b;
            true
        } else {
            false
        }
    });

    let extracted: Vec<&str> = spans.iter().map(|(_, s)| *s).collect();
    let joined = extracted.join(" ");

    let result = if strip {
        strip_markers(&joined)
    } else {
        joined
    };

    Some(result)
}

/// Extract marked text from a block's lines.
///
/// For each line, checks if it contains bold/highlight spans. Depending
/// on `keep_paragraph`:
/// - `false`: Only the extracted spans (with any list/blockquote prefix) are output.
/// - `true`: If *any* line has a match, all lines are returned verbatim.
///
/// # Arguments
///
/// * `lines` - The block's source lines
/// * `mode` - Filter mode (bold, highlight, or both)
/// * `strip` - Whether to strip markers from output
/// * `keep_paragraph` - Whether to keep the entire paragraph if any match found
///
/// # Returns
///
/// A `Vec<String>` of output lines (may be empty if no matches).
pub fn extract_from_lines(
    lines: &[String],
    mode: FilterMode,
    strip: bool,
    keep_paragraph: bool,
) -> Vec<String> {
    if keep_paragraph {
        let has_any = lines.iter().any(|line| {
            let content = strip_line_prefix(line);
            extract_marked_text(content, mode, false).is_some()
        });
        if has_any {
            return lines.to_vec();
        }
        return Vec::new();
    }

    let mut output = Vec::new();
    for line in lines {
        let (prefix, content) = split_line_prefix(line);
        if let Some(extracted) = extract_marked_text(content, mode, strip) {
            if prefix.is_empty() {
                output.push(extracted);
            } else {
                output.push(format!("{prefix}{extracted}"));
            }
        }
    }
    output
}

/// Extract marked text from callout block lines.
///
/// Preserves the callout structure (`> ` prefix and `[!type]` header).
/// Lines without marked text are dropped (unless `keep_paragraph` is set).
///
/// # Arguments
///
/// * `lines` - The callout block's lines (each starting with `> `)
/// * `mode` - Filter mode
/// * `strip` - Whether to strip markers
/// * `keep_paragraph` - If true and any line has a match, keep all lines
///
/// # Returns
///
/// A `Vec<String>` preserving the callout prefix structure.
pub fn extract_from_callout(
    lines: &[String],
    mode: FilterMode,
    strip: bool,
    keep_paragraph: bool,
) -> Vec<String> {
    if keep_paragraph {
        let has_any = lines.iter().any(|line| {
            let content = strip_line_prefix(line);
            extract_marked_text(content, mode, false).is_some()
        });
        if has_any {
            return lines.to_vec();
        }
        return Vec::new();
    }

    // Check if any line in the callout has marked text
    let has_any = lines.iter().any(|line| {
        let content = strip_line_prefix(line);
        extract_marked_text(content, mode, false).is_some()
    });
    if !has_any {
        return Vec::new();
    }

    let mut output = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let content = strip_line_prefix(line);
        let has_match = extract_marked_text(content, mode, false).is_some();

        // Always keep the callout header (first line) when there's content
        if i == 0 || has_match {
            if strip && has_match {
                let extracted = extract_marked_text(content, mode, true)
                    .expect("already confirmed match exists");
                let prefix_len = line.len() - content.len();
                output.push(format!("{}{}", &line[..prefix_len], extracted));
            } else {
                output.push(line.clone());
            }
        }
    }
    output
}

/// Remove `**` and `==` delimiters from text.
fn strip_markers(text: &str) -> String {
    text.replace("**", "").replace("==", "")
}

/// Strip blockquote `> ` or list item prefix to get the content.
fn strip_line_prefix(line: &str) -> &str {
    if let Some(rest) = line.strip_prefix("> ") {
        return rest;
    }
    if let Some(rest) = line.strip_prefix('>') {
        return rest;
    }
    line
}

/// Split a line into its structural prefix and content.
///
/// Returns `(prefix, content)` where prefix includes the `> ` or `- ` etc.
fn split_line_prefix(line: &str) -> (&str, &str) {
    // Blockquote prefix
    if let Some(rest) = line.strip_prefix("> ") {
        let prefix_len = line.len() - rest.len();
        return (&line[..prefix_len], rest);
    }
    if let Some(rest) = line.strip_prefix('>') {
        let prefix_len = line.len() - rest.len();
        return (&line[..prefix_len], rest);
    }

    // List item prefix: "- ", "* ", "+ ", or "N. " (with optional leading whitespace)
    let trimmed = line.trim_start();
    let leading_ws = line.len() - trimmed.len();

    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0];
        if (first == b'-' || first == b'*' || first == b'+') && trimmed.as_bytes()[1] == b' ' {
            let prefix_len = leading_ws + 2;
            return (&line[..prefix_len], &line[prefix_len..]);
        }
    }

    // Ordered list: "1. ", "10. ", etc.
    let digit_count = trimmed.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digit_count > 0 && trimmed[digit_count..].starts_with(". ") {
        let prefix_len = leading_ws + digit_count + 2;
        return (&line[..prefix_len], &line[prefix_len..]);
    }

    ("", line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FilterMode;

    // -- Task 12: basic bold extraction --

    #[test]
    fn extract_single_bold_span() {
        let result = extract_marked_text("Hello **world** goodbye", FilterMode::Both, false);
        assert_eq!(result, Some("**world**".to_string()));
    }

    #[test]
    fn extract_multiple_bold_spans() {
        let result = extract_marked_text("A **first** B **second** C", FilterMode::Both, false);
        assert_eq!(result, Some("**first** **second**".to_string()));
    }

    #[test]
    fn no_bold_returns_none() {
        let result = extract_marked_text("No bold here", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn entire_line_is_bold() {
        let result = extract_marked_text("**The whole line is bold.**", FilterMode::Both, false);
        assert_eq!(result, Some("**The whole line is bold.**".to_string()));
    }

    #[test]
    fn bold_with_inline_code() {
        let result = extract_marked_text("**Use `println!` for output.**", FilterMode::Both, false);
        assert_eq!(result, Some("**Use `println!` for output.**".to_string()));
    }

    #[test]
    fn bold_with_inline_math() {
        let result = extract_marked_text(
            "**The equation $x^2 + 1 = 0$ has no real root.**",
            FilterMode::Both,
            false,
        );
        assert_eq!(
            result,
            Some("**The equation $x^2 + 1 = 0$ has no real root.**".to_string())
        );
    }

    #[test]
    fn unclosed_bold_is_literal() {
        let result = extract_marked_text("Hello ** world", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn empty_bold_ignored() {
        let result = extract_marked_text("Before **** after", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    // -- Task 13: highlight extraction --

    #[test]
    fn extract_highlight_span() {
        let result = extract_marked_text("Hello ==world== goodbye", FilterMode::Both, false);
        assert_eq!(result, Some("==world==".to_string()));
    }

    #[test]
    fn extract_multiple_highlights() {
        let result = extract_marked_text("A ==first== B ==second== C", FilterMode::Both, false);
        assert_eq!(result, Some("==first== ==second==".to_string()));
    }

    #[test]
    fn highlight_only_mode_ignores_bold() {
        let result = extract_marked_text(
            "**bold** and ==highlight==",
            FilterMode::HighlightOnly,
            false,
        );
        assert_eq!(result, Some("==highlight==".to_string()));
    }

    #[test]
    fn bold_only_mode_ignores_highlight() {
        let result = extract_marked_text("**bold** and ==highlight==", FilterMode::BoldOnly, false);
        assert_eq!(result, Some("**bold**".to_string()));
    }

    #[test]
    fn unclosed_highlight_is_literal() {
        let result = extract_marked_text("Hello == world", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    // -- Task 14: nested bold and highlight --

    #[test]
    fn nested_highlight_inside_bold() {
        let result = extract_marked_text(
            "**==For every successful language==**",
            FilterMode::Both,
            false,
        );
        assert_eq!(
            result,
            Some("**==For every successful language==**".to_string())
        );
    }

    #[test]
    fn nested_highlight_inside_bold_with_extra() {
        let result = extract_marked_text(
            "**==highlighted part== and bold part**",
            FilterMode::Both,
            false,
        );
        assert_eq!(
            result,
            Some("**==highlighted part== and bold part**".to_string())
        );
    }

    #[test]
    fn nested_bold_only_mode_captures_full_span() {
        let result = extract_marked_text(
            "**==For every successful language==**",
            FilterMode::BoldOnly,
            false,
        );
        assert_eq!(
            result,
            Some("**==For every successful language==**".to_string())
        );
    }

    #[test]
    fn nested_highlight_only_mode_extracts_inner() {
        let result = extract_marked_text(
            "**==For every successful language==**",
            FilterMode::HighlightOnly,
            false,
        );
        assert_eq!(
            result,
            Some("==For every successful language==".to_string())
        );
    }

    #[test]
    fn real_example_bold_with_nested_highlight() {
        let input = "**==For every successful general-purpose language, there are a thousand successful niche ones. We used to call them \"little languages\", but inflation in the jargon economy led to the name \"domain-specific languages\".== These are pidgins tailorbuilt to a specific task. Think application scripting languages, template engines, markup formats, and configuration files.**";
        let result = extract_marked_text(input, FilterMode::Both, false);
        assert_eq!(result, Some(input.to_string()));
    }

    #[test]
    fn real_example_highlight_only_nested() {
        let input = "**==For every successful general-purpose language, there are a thousand successful niche ones.== Rest of bold.**";
        let result = extract_marked_text(input, FilterMode::HighlightOnly, false);
        assert_eq!(
            result,
            Some("==For every successful general-purpose language, there are a thousand successful niche ones.==".to_string())
        );
    }

    // -- Task 15: strip markers --

    #[test]
    fn strip_bold_markers() {
        let result = extract_marked_text("Hello **world** bye", FilterMode::Both, true);
        assert_eq!(result, Some("world".to_string()));
    }

    #[test]
    fn strip_highlight_markers() {
        let result = extract_marked_text("Hello ==world== bye", FilterMode::Both, true);
        assert_eq!(result, Some("world".to_string()));
    }

    #[test]
    fn strip_nested_markers() {
        let result = extract_marked_text("**==text==**", FilterMode::Both, true);
        assert_eq!(result, Some("text".to_string()));
    }

    #[test]
    fn strip_multiple_spans() {
        let result = extract_marked_text("**bold** and ==highlight==", FilterMode::Both, true);
        assert_eq!(result, Some("bold highlight".to_string()));
    }

    // -- Task 16: escaped markers --

    #[test]
    fn escaped_bold_is_literal() {
        let result = extract_marked_text(r"Hello \*\*not bold\*\* bye", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn escaped_highlight_is_literal() {
        let result =
            extract_marked_text(r"Hello \=\=not highlight\=\= bye", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    // -- Task 17: process block content --

    #[test]
    fn extract_from_paragraph_lines() {
        let lines = vec![
            "Normal text.".to_string(),
            "**Bold text.** More normal.".to_string(),
            "Also normal.".to_string(),
        ];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert_eq!(result, vec!["**Bold text.**"]);
    }

    #[test]
    fn extract_from_paragraph_keep_paragraph() {
        let lines = vec![
            "Normal text.".to_string(),
            "**Bold text.** More normal.".to_string(),
            "Also normal.".to_string(),
        ];
        let result = extract_from_lines(&lines, FilterMode::Both, false, true);
        assert_eq!(
            result,
            vec![
                "Normal text.",
                "**Bold text.** More normal.",
                "Also normal."
            ]
        );
    }

    #[test]
    fn extract_from_paragraph_no_bold() {
        let lines = vec!["No bold here.".to_string(), "None here either.".to_string()];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert!(result.is_empty());
    }

    #[test]
    fn extract_from_list_item() {
        let lines = vec!["- **For every output tolerance $\\varepsilon$, there is some input tolerance $\\delta$ that works.**".to_string()];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert_eq!(
            result,
            vec![
                "- **For every output tolerance $\\varepsilon$, there is some input tolerance $\\delta$ that works.**"
            ]
        );
    }

    #[test]
    fn extract_from_blockquote() {
        let lines = vec![
            "> **A compiler translates code.**".to_string(),
            "> Normal text.".to_string(),
        ];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert_eq!(result, vec!["> **A compiler translates code.**"]);
    }

    // -- Task 18: callout content extraction --

    #[test]
    fn extract_from_callout_lines() {
        let lines = vec![
            "> [!info] Curry-Howard isomorphism".to_string(),
            "> Normal callout text.".to_string(),
            "> **A logical statement** is a kind of **specification**".to_string(),
        ];
        let result = extract_from_callout(&lines, FilterMode::Both, false, false);
        assert_eq!(
            result,
            vec![
                "> [!info] Curry-Howard isomorphism",
                "> **A logical statement** is a kind of **specification**",
            ]
        );
    }

    #[test]
    fn extract_from_callout_no_bold() {
        let lines = vec!["> [!info] Title".to_string(), "> No bold here.".to_string()];
        let result = extract_from_callout(&lines, FilterMode::Both, false, false);
        assert!(result.is_empty());
    }

    #[test]
    fn extract_from_callout_keep_paragraph() {
        let lines = vec![
            "> [!info] Title".to_string(),
            "> Normal text.".to_string(),
            "> **Bold text.**".to_string(),
        ];
        let result = extract_from_callout(&lines, FilterMode::Both, false, true);
        assert_eq!(result, lines);
    }

    #[test]
    fn extract_from_callout_with_bold_in_header() {
        let lines = vec![
            "> [!info] **Important title**".to_string(),
            "> Normal text.".to_string(),
        ];
        let result = extract_from_callout(&lines, FilterMode::Both, false, false);
        assert_eq!(result, vec!["> [!info] **Important title**"]);
    }
}
