use crate::types::{Block, BlockKind};

/// Internal state machine for multi-line block tracking.
///
/// When in `Normal` state, each line is classified independently.
/// Other states accumulate lines into the current block until
/// the closing delimiter is found.
#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    /// Not inside any multi-line construct.
    Normal,
    /// Inside a fenced code block. Stores the fence character and
    /// minimum count needed to close it.
    InCodeFence { fence_char: u8, fence_count: usize },
    /// Inside a display math block (`$$ ... $$`).
    InDisplayMath,
    /// Inside an HTML `<table>...</table>` block.
    InHtmlTable,
    /// Inside a callout block (`> [!type] ...`).
    InCallout,
}

/// Parse a sequence of lines into classified blocks.
///
/// This is Pass 1 of the two-pass architecture. Uses a state machine to
/// correctly handle multi-line constructs (code fences, display math,
/// HTML tables, callouts) without false-positive classification of their
/// inner lines.
///
/// # Arguments
///
/// * `lines` - The input file split into lines (no trailing newlines).
///
/// # Returns
///
/// A `Vec<Block>` where consecutive lines of the same kind are grouped.
pub fn parse_blocks(lines: &[String]) -> Vec<Block> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut blocks: Vec<Block> = Vec::new();
    let mut state = ParserState::Normal;

    for line in lines {
        match &state {
            ParserState::Normal => {
                state = classify_normal_line(line, &mut blocks);
            }
            ParserState::InCodeFence {
                fence_char,
                fence_count,
            } => {
                if let Some(last) = blocks.last_mut() {
                    last.lines.push(line.clone());
                }
                if is_code_fence_close(line, *fence_char, *fence_count) {
                    state = ParserState::Normal;
                }
            }
            ParserState::InDisplayMath => {
                if let Some(last) = blocks.last_mut() {
                    last.lines.push(line.clone());
                }
                if is_display_math_delimiter(line) {
                    state = ParserState::Normal;
                }
            }
            ParserState::InHtmlTable => {
                if let Some(last) = blocks.last_mut() {
                    last.lines.push(line.clone());
                }
                if is_html_table_close(line) {
                    state = ParserState::Normal;
                }
            }
            ParserState::InCallout => {
                if is_blockquote_line(line) {
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                } else {
                    // Callout ended — re-classify this line in Normal state
                    state = classify_normal_line(line, &mut blocks);
                }
            }
        }
    }

    blocks
}

/// Process a single line under Normal-state classification rules.
///
/// Returns the new parser state (may transition to a multi-line block).
fn classify_normal_line(line: &str, blocks: &mut Vec<Block>) -> ParserState {
    // Fenced code block opening
    if let Some((fence_char, fence_count)) = detect_code_fence_open(line) {
        blocks.push(Block {
            kind: BlockKind::CodeBlock,
            lines: vec![line.to_string()],
        });
        return ParserState::InCodeFence {
            fence_char,
            fence_count,
        };
    }

    // Display math opening
    if is_display_math_delimiter(line) {
        blocks.push(Block {
            kind: BlockKind::DisplayMath,
            lines: vec![line.to_string()],
        });
        return ParserState::InDisplayMath;
    }

    // HTML table opening
    if is_html_table_open(line) {
        let closes = is_html_table_close(line);
        blocks.push(Block {
            kind: BlockKind::HtmlTable,
            lines: vec![line.to_string()],
        });
        if closes {
            return ParserState::Normal;
        }
        return ParserState::InHtmlTable;
    }

    // Callout: > [!type]
    if is_callout_start(line) {
        blocks.push(Block {
            kind: BlockKind::Callout,
            lines: vec![line.to_string()],
        });
        return ParserState::InCallout;
    }

    // Blockquote: > ... (not a callout)
    if is_blockquote_line(line) {
        push_or_merge(blocks, BlockKind::Blockquote, line.to_string());
        return ParserState::Normal;
    }

    // Image line
    if is_image_line(line) {
        push_or_merge(blocks, BlockKind::Image, line.to_string());
        return ParserState::Normal;
    }

    // List item
    if is_list_item(line) {
        push_or_merge(blocks, BlockKind::ListItem, line.to_string());
        return ParserState::Normal;
    }

    // Markdown table detection: separator row reclassifies previous paragraph
    if is_table_separator(line)
        && let Some(last) = blocks.last_mut()
        && last.kind == BlockKind::Paragraph
        && last.lines.iter().all(|l| is_table_row(l))
    {
        last.kind = BlockKind::MarkdownTable;
        last.lines.push(line.to_string());
        return ParserState::Normal;
    }

    // Markdown table data row continuation
    if is_table_row(line)
        && let Some(last) = blocks.last_mut()
        && last.kind == BlockKind::MarkdownTable
    {
        last.lines.push(line.to_string());
        return ParserState::Normal;
    }

    let kind = classify_line(line);
    push_or_merge(blocks, kind, line.to_string());
    ParserState::Normal
}

/// Classify a single line in Normal state (simple, non-stateful checks).
fn classify_line(line: &str) -> BlockKind {
    if line.trim().is_empty() {
        return BlockKind::BlankLine;
    }

    // ATX heading: 1-6 '#' followed by a space (or end of line for bare "#")
    if line.starts_with('#') {
        let hash_count = line.bytes().take_while(|&b| b == b'#').count();
        if hash_count <= 6 {
            let rest = &line[hash_count..];
            if rest.is_empty() || rest.starts_with(' ') {
                return BlockKind::Heading {
                    level: hash_count as u8,
                };
            }
        }
    }

    BlockKind::Paragraph
}

/// Detect if a line opens a fenced code block.
///
/// Returns `Some((fence_char, fence_count))` if the line starts with 3+
/// backticks or tildes.
fn detect_code_fence_open(line: &str) -> Option<(u8, usize)> {
    let trimmed = line.trim_start();
    let first_byte = trimmed.as_bytes().first()?;

    if *first_byte != b'`' && *first_byte != b'~' {
        return None;
    }

    let fence_char = *first_byte;
    let count = trimmed.bytes().take_while(|&b| b == fence_char).count();

    if count >= 3 {
        Some((fence_char, count))
    } else {
        None
    }
}

/// Check if a line closes a code fence.
///
/// A closing fence must use the same character as the opening fence,
/// with at least as many repetitions, and nothing else on the line
/// besides optional trailing whitespace.
fn is_code_fence_close(line: &str, fence_char: u8, min_count: usize) -> bool {
    let trimmed = line.trim_start();
    let first_byte = match trimmed.as_bytes().first() {
        Some(&b) => b,
        None => return false,
    };

    if first_byte != fence_char {
        return false;
    }

    let count = trimmed.bytes().take_while(|&b| b == fence_char).count();
    if count < min_count {
        return false;
    }

    trimmed[count..].trim().is_empty()
}

/// Check if a line is a display math delimiter (`$$`).
fn is_display_math_delimiter(line: &str) -> bool {
    line.trim() == "$$"
}

/// Check if a line opens an HTML table.
fn is_html_table_open(line: &str) -> bool {
    let trimmed = line.trim_start().to_ascii_lowercase();
    trimmed.starts_with("<table")
}

/// Check if a line closes an HTML table.
fn is_html_table_close(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.contains("</table>")
}

/// Check if a line starts a callout block (`> [!type]`).
fn is_callout_start(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("> [!")
}

/// Check if a line is a blockquote continuation (`> ...` or just `>`).
fn is_blockquote_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with('>')
}

/// Check if a line is an image (Obsidian `![[...]]` or standard `![...](...)` ).
fn is_image_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with("![[") && trimmed.ends_with("]]") {
        return true;
    }
    if trimmed.starts_with("![") && trimmed.contains("](") && trimmed.ends_with(')') {
        return true;
    }
    false
}

/// Check if a line is a list item (unordered or ordered).
///
/// Unordered: starts with `-`, `*`, or `+` followed by a space.
/// Ordered: starts with digits followed by `.` and a space.
/// Leading whitespace is allowed (for nested lists).
fn is_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0];
        if (first == b'-' || first == b'+') && trimmed.as_bytes()[1] == b' ' {
            return true;
        }
        // Asterisk list item — but NOT bold marker "**"
        if first == b'*' && trimmed.as_bytes()[1] == b' ' {
            return true;
        }
    }
    // Ordered: "1. ", "2. ", "10. ", etc.
    let digit_count = trimmed.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digit_count > 0 && trimmed[digit_count..].starts_with(". ") {
        return true;
    }
    false
}

/// Check if a line looks like a Markdown table separator row.
///
/// Separator rows contain only `|`, `-`, `:`, and whitespace.
fn is_table_separator(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.contains('|') || !trimmed.contains('-') {
        return false;
    }
    trimmed.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

/// Check if a line looks like a Markdown table row (starts and ends with `|`).
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}

/// Append a line to the last block if it has the same kind,
/// otherwise start a new block.
fn push_or_merge(blocks: &mut Vec<Block>, kind: BlockKind, line: String) {
    if let Some(last) = blocks.last_mut()
        && last.kind == kind
    {
        last.lines.push(line);
        return;
    }
    blocks.push(Block {
        kind,
        lines: vec![line],
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BlockKind;

    /// Helper: parse a string and return the block kinds + their line counts.
    fn parse_to_kinds(input: &str) -> Vec<(BlockKind, usize)> {
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let blocks = parse_blocks(&lines);
        blocks
            .iter()
            .map(|b| (b.kind.clone(), b.lines.len()))
            .collect()
    }

    // -- Task 5: blank lines and paragraphs --

    #[test]
    fn empty_input() {
        assert_eq!(parse_to_kinds(""), Vec::<(BlockKind, usize)>::new());
    }

    #[test]
    fn single_paragraph() {
        let result = parse_to_kinds("Hello world");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }

    #[test]
    fn paragraph_separated_by_blank_line() {
        let result = parse_to_kinds("First paragraph\n\nSecond paragraph");
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::BlankLine, 1),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn multiple_blank_lines_merge() {
        let result = parse_to_kinds("Text\n\n\n\nMore text");
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::BlankLine, 3),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    // -- Task 6: headings --

    #[test]
    fn heading_levels() {
        let result = parse_to_kinds("# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6");
        assert_eq!(
            result,
            vec![
                (BlockKind::Heading { level: 1 }, 1),
                (BlockKind::Heading { level: 2 }, 1),
                (BlockKind::Heading { level: 3 }, 1),
                (BlockKind::Heading { level: 4 }, 1),
                (BlockKind::Heading { level: 5 }, 1),
                (BlockKind::Heading { level: 6 }, 1),
            ]
        );
    }

    #[test]
    fn heading_requires_space_after_hash() {
        let result = parse_to_kinds("#nospace");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }

    #[test]
    fn seven_hashes_is_paragraph() {
        let result = parse_to_kinds("####### too many");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }

    // -- Task 7: fenced code blocks --

    #[test]
    fn fenced_code_block_backticks() {
        let input = "before\n```\ncode line\n```\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::CodeBlock, 3),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn fenced_code_block_tildes() {
        let input = "~~~\ncode\n~~~";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn fenced_code_block_with_language() {
        let input = "```rust\nfn main() {}\n```";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn fenced_code_four_backticks() {
        let input = "````\ncode with ``` inside\n````";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn fenced_code_five_backticks() {
        let input = "`````\ncode with ```` inside\n`````";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn fenced_code_three_backticks_not_closed_by_two() {
        let input = "```\ncode\n``\nmore code\n```";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 5)]);
    }

    #[test]
    fn unclosed_code_fence_at_eof() {
        let input = "```\ncode\nmore code";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn bold_inside_code_block_not_extracted() {
        let input = "```\n**not bold**\n```";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    // -- Task 8: display math --

    #[test]
    fn display_math_block() {
        let input = "before\n$$\nx^2 + y^2 = z^2\n$$\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::DisplayMath, 3),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn display_math_multiline() {
        let input = "$$\na = b\nc = d\n$$";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::DisplayMath, 4)]);
    }

    #[test]
    fn display_math_unclosed_at_eof() {
        let input = "$$\nmath\nmore math";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::DisplayMath, 3)]);
    }

    #[test]
    fn inline_dollar_not_display_math() {
        let input = "The value is $x^2$ here";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }

    // -- Task 9: HTML tables --

    #[test]
    fn html_table() {
        let input = "before\n<table>\n<tr><td>cell</td></tr>\n</table>\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::HtmlTable, 3),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn html_table_with_attributes() {
        let input = "<table style=\"margin: auto;\">\n<tr><td>x</td></tr>\n</table>";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::HtmlTable, 3)]);
    }

    #[test]
    fn html_table_unclosed_at_eof() {
        let input = "<table>\n<tr><td>cell</td></tr>";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::HtmlTable, 2)]);
    }

    // -- Task 10: callout blocks and blockquotes --

    #[test]
    fn callout_block() {
        let input = "> [!info] Title\n> Content line\n> More content\n\nAfter";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![
                (BlockKind::Callout, 3),
                (BlockKind::BlankLine, 1),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn callout_success_type() {
        let input = "> [!success] Answer\n> The answer is 42.";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Callout, 2)]);
    }

    #[test]
    fn regular_blockquote_not_callout() {
        let input = "> Just a quote\n> continuation";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Blockquote, 2)]);
    }

    #[test]
    fn blockquote_single_line() {
        let input = "> A single line quote";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Blockquote, 1)]);
    }

    #[test]
    fn callout_ends_at_non_quote_line() {
        let input = "> [!info] Title\n> Line 1\nNot a quote line";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![(BlockKind::Callout, 2), (BlockKind::Paragraph, 1),]
        );
    }

    #[test]
    fn nested_blockquote_in_callout() {
        let input = "> [!info] Title\n> > Nested quote";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Callout, 2)]);
    }

    // -- Task 11: images, list items, Markdown tables --

    #[test]
    fn obsidian_image() {
        let result = parse_to_kinds("![[my-image.png]]");
        assert_eq!(result, vec![(BlockKind::Image, 1)]);
    }

    #[test]
    fn standard_markdown_image() {
        let result = parse_to_kinds("![alt text](https://example.com/img.jpg)");
        assert_eq!(result, vec![(BlockKind::Image, 1)]);
    }

    #[test]
    fn image_with_surrounding_text() {
        let result = parse_to_kinds("text before\n![img](url)\ntext after");
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::Image, 1),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn unordered_list_item_dash() {
        let result = parse_to_kinds("- item one\n- item two");
        assert_eq!(result, vec![(BlockKind::ListItem, 2)]);
    }

    #[test]
    fn unordered_list_item_asterisk() {
        let result = parse_to_kinds("* item one");
        assert_eq!(result, vec![(BlockKind::ListItem, 1)]);
    }

    #[test]
    fn unordered_list_item_plus() {
        let result = parse_to_kinds("+ item one");
        assert_eq!(result, vec![(BlockKind::ListItem, 1)]);
    }

    #[test]
    fn ordered_list_item() {
        let result = parse_to_kinds("1. First\n2. Second\n3. Third");
        assert_eq!(result, vec![(BlockKind::ListItem, 3)]);
    }

    #[test]
    fn indented_list_item() {
        let result = parse_to_kinds("  - nested item");
        assert_eq!(result, vec![(BlockKind::ListItem, 1)]);
    }

    #[test]
    fn markdown_table() {
        let input = "| Col A | Col B |\n|-------|-------|\n| val   | val   |";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::MarkdownTable, 3)]);
    }

    #[test]
    fn markdown_table_with_surrounding() {
        let input = "text\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\nmore text";
        let result = parse_to_kinds(input);
        assert_eq!(
            result,
            vec![
                (BlockKind::Paragraph, 1),
                (BlockKind::BlankLine, 1),
                (BlockKind::MarkdownTable, 3),
                (BlockKind::BlankLine, 1),
                (BlockKind::Paragraph, 1),
            ]
        );
    }

    #[test]
    fn pipe_in_paragraph_not_table() {
        let result = parse_to_kinds("value a | value b");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }
}
