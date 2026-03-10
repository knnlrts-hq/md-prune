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

    let kind = classify_line(line);
    push_or_merge(blocks, kind, line.to_string());
    ParserState::Normal
}

/// Classify a single line in Normal state.
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
/// backticks or tildes. The `fence_count` is the number of fence characters.
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

/// Append a line to the last block if it has the same kind,
/// otherwise start a new block.
fn push_or_merge(blocks: &mut Vec<Block>, kind: BlockKind, line: String) {
    // Rust note: `last_mut()` returns Option<&mut Block> — a mutable
    // reference to the last element, if one exists.
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
}
