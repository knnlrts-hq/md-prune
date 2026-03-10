# md-prune Implementation Plan — Part 2: Block Parser (Pass 1)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the line-by-line state machine that classifies every line of a Markdown file into a `Block` with a `BlockKind`.

**Prerequisite:** Part 1 complete (types, errors, CLI all compiling and tested).

**Key file:** `src/parser.rs`

---

## How the parser works

The parser reads the input as a `Vec<String>` of lines and produces a `Vec<Block>`. It maintains a *state machine* that tracks whether we're currently inside a multi-line construct (code fence, display math, HTML table, or callout). The state determines how each line is classified.

**State transitions:**

```
Normal ──┬── sees ``` ──────> InCodeFence(n)  ──── sees ``` (≥n) ──> Normal
         ├── sees $$  ──────> InDisplayMath    ──── sees $$       ──> Normal
         ├── sees <table ──> InHtmlTable       ──── sees </table> ──> Normal
         └── sees > [! ────> InCallout         ──── blank line    ──> Normal
```

While in any of these states, lines are accumulated into the current block without further classification. This prevents false positives (e.g., `**bold**` inside a code block should NOT be extracted).

---

## Task 5: Parser — blank lines and paragraphs

Start with the simplest cases. We'll add more block types incrementally.

**Files:**
- Create: `src/parser.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Write the failing tests**

Create `src/parser.rs` with:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BlockKind;

    /// Helper: parse a string and return the block kinds + their line counts.
    fn parse_to_kinds(input: &str) -> Vec<(BlockKind, usize)> {
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let blocks = parse_blocks(&lines);
        blocks.iter().map(|b| (b.kind.clone(), b.lines.len())).collect()
    }

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
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::BlankLine, 1),
            (BlockKind::Paragraph, 1),
        ]);
    }

    #[test]
    fn multiple_blank_lines_merge() {
        let result = parse_to_kinds("Text\n\n\n\nMore text");
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::BlankLine, 3),
            (BlockKind::Paragraph, 1),
        ]);
    }
}
```

**Step 2: Run tests to verify they fail**

```bash
cargo test --lib parser
```

Expected: Compilation error — `parse_blocks` not defined.

**Step 3: Write minimal implementation**

Add above the test module in `src/parser.rs`:

```rust
use crate::types::{Block, BlockKind};

/// Parse a sequence of lines into classified blocks.
///
/// This is Pass 1 of the two-pass architecture. Each contiguous group of
/// lines sharing the same classification becomes one [`Block`].
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
    // Rust note: `&String` auto-derefs to `&str` when you call str methods.
    for line in lines {
        let kind = classify_line(line);
        push_or_merge(&mut blocks, kind, line.clone());
    }
    blocks
}

/// Classify a single line into a block kind.
///
/// This is the "Normal" state classifier. Multi-line block states
/// (code fence, display math, etc.) override this in the main loop.
fn classify_line(line: &str) -> BlockKind {
    if line.trim().is_empty() {
        return BlockKind::BlankLine;
    }
    BlockKind::Paragraph
}

/// Append a line to the last block if it has the same kind,
/// otherwise start a new block.
fn push_or_merge(blocks: &mut Vec<Block>, kind: BlockKind, line: String) {
    // Rust note: `last_mut()` returns Option<&mut Block> — a mutable
    // reference to the last element, if one exists.
    if let Some(last) = blocks.last_mut() {
        if last.kind == kind {
            last.lines.push(line);
            return;
        }
    }
    blocks.push(Block {
        kind,
        lines: vec![line],
    });
}
```

**Step 4: Register the module**

Add `mod parser;` to `src/main.rs`:

```rust
mod cli;
mod errors;
mod parser;
mod types;
```

**Step 5: Run tests**

```bash
cargo test --lib parser
```

Expected: All 4 tests pass.

**Step 6: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs src/main.rs
git commit -m "feat: parser skeleton with blank line and paragraph detection"
```

---

## Task 6: Parser — headings

**Files:**
- Modify: `src/parser.rs`

**Step 1: Add failing tests**

Add these tests to the existing `mod tests` block:

```rust
    #[test]
    fn heading_levels() {
        let result = parse_to_kinds("# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6");
        assert_eq!(result, vec![
            (BlockKind::Heading { level: 1 }, 1),
            (BlockKind::Heading { level: 2 }, 1),
            (BlockKind::Heading { level: 3 }, 1),
            (BlockKind::Heading { level: 4 }, 1),
            (BlockKind::Heading { level: 5 }, 1),
            (BlockKind::Heading { level: 6 }, 1),
        ]);
    }

    #[test]
    fn heading_requires_space_after_hash() {
        // "#nospace" is NOT a heading — CommonMark requires a space
        let result = parse_to_kinds("#nospace");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }

    #[test]
    fn seven_hashes_is_paragraph() {
        // Only levels 1-6 are valid
        let result = parse_to_kinds("####### too many");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib parser
```

Expected: `heading_levels` fails because all lines classify as `Paragraph`.

**Step 3: Update `classify_line`**

Add heading detection to `classify_line`, before the final `Paragraph` fallback:

```rust
fn classify_line(line: &str) -> BlockKind {
    if line.trim().is_empty() {
        return BlockKind::BlankLine;
    }

    // ATX heading: 1-6 '#' followed by a space (or end of line for bare "#")
    if line.starts_with('#') {
        let hash_count = line.bytes().take_while(|&b| b == b'#').count();
        if hash_count <= 6 {
            // Must have a space after hashes, or be exactly "###..." with nothing after
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
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 7 tests pass (4 old + 3 new).

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser detects ATX headings (levels 1-6)"
```

---

## Task 7: Parser — fenced code blocks

This is the first multi-line state. The parser must track whether we're inside a code fence.

**Files:**
- Modify: `src/parser.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn fenced_code_block_backticks() {
        let input = "before\n```\ncode line\n```\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::CodeBlock, 3), // opening fence + code + closing fence
            (BlockKind::Paragraph, 1),
        ]);
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
        // 4 backticks open — must close with 4+ backticks
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
        // ``` is NOT closed by ``
        let input = "```\ncode\n``\nmore code\n```";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 5)]);
    }

    #[test]
    fn unclosed_code_fence_at_eof() {
        let input = "```\ncode\nmore code";
        let result = parse_to_kinds(input);
        // Entire remaining content is the code block
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }

    #[test]
    fn bold_inside_code_block_not_extracted() {
        // **bold** inside a code block should NOT become a Paragraph
        let input = "```\n**not bold**\n```";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::CodeBlock, 3)]);
    }
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib parser
```

Expected: Code fence lines are classified as `Paragraph`.

**Step 3: Refactor parser to use a state machine**

This is a significant refactor. Replace the `parse_blocks` function and add state tracking:

```rust
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
    InCodeFence {
        fence_char: u8,
        fence_count: usize,
    },
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
                // Check for code fence opening
                if let Some((fence_char, fence_count)) = detect_code_fence_open(line) {
                    state = ParserState::InCodeFence {
                        fence_char,
                        fence_count,
                    };
                    blocks.push(Block {
                        kind: BlockKind::CodeBlock,
                        lines: vec![line.clone()],
                    });
                    continue;
                }

                let kind = classify_line(line);
                push_or_merge(&mut blocks, kind, line.clone());
            }
            ParserState::InCodeFence {
                fence_char,
                fence_count,
            } => {
                // Closing fence: same char, count >= opening, nothing else
                // (besides optional whitespace)
                if is_code_fence_close(line, *fence_char, *fence_count) {
                    // Add closing fence to current block, return to Normal
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                    state = ParserState::Normal;
                } else {
                    // Accumulate line into current code block
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                }
            }
        }
    }

    blocks
}

/// Detect if a line opens a fenced code block.
///
/// Returns `Some((fence_char, fence_count))` if the line starts with 3+
/// backticks or tildes. The `fence_count` is the number of fence characters.
///
/// # Examples
///
/// - `` ``` `` → `Some((b'`', 3))`
/// - `` ```rust `` → `Some((b'`', 3))`
/// - `~~~` → `Some((b'~', 3))`
/// - `` `` `` (only 2) → `None`
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

    // Nothing else after the fence chars (except whitespace)
    trimmed[count..].trim().is_empty()
}

/// Classify a single line in Normal state.
fn classify_line(line: &str) -> BlockKind {
    if line.trim().is_empty() {
        return BlockKind::BlankLine;
    }

    // ATX heading: 1-6 '#' followed by a space
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

/// Append a line to the last block if it has the same kind,
/// otherwise start a new block.
fn push_or_merge(blocks: &mut Vec<Block>, kind: BlockKind, line: String) {
    if let Some(last) = blocks.last_mut() {
        if last.kind == kind {
            last.lines.push(line);
            return;
        }
    }
    blocks.push(Block {
        kind,
        lines: vec![line],
    });
}
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 15 tests pass (7 old + 8 new).

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser state machine for fenced code blocks (3+ backticks/tildes)"
```

---

## Task 8: Parser — display math blocks

**Files:**
- Modify: `src/parser.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn display_math_block() {
        let input = "before\n$$\nx^2 + y^2 = z^2\n$$\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::DisplayMath, 3),
            (BlockKind::Paragraph, 1),
        ]);
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
        // Single $ is inline math, not display math
        let input = "The value is $x^2$ here";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib parser
```

**Step 3: Add `InDisplayMath` state**

Add a new variant to `ParserState`:

```rust
enum ParserState {
    Normal,
    InCodeFence {
        fence_char: u8,
        fence_count: usize,
    },
    /// Inside a display math block (`$$ ... $$`).
    InDisplayMath,
}
```

Add a helper to detect `$$` lines:

```rust
/// Check if a line is a display math delimiter (`$$`).
///
/// The line must contain `$$` with only optional whitespace around it.
fn is_display_math_delimiter(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed == "$$"
}
```

In the `parse_blocks` main loop, add display math handling in the `Normal` arm (after the code fence check, before `classify_line`):

```rust
                // Check for display math opening
                if is_display_math_delimiter(line) {
                    state = ParserState::InDisplayMath;
                    blocks.push(Block {
                        kind: BlockKind::DisplayMath,
                        lines: vec![line.clone()],
                    });
                    continue;
                }
```

Add the `InDisplayMath` match arm:

```rust
            ParserState::InDisplayMath => {
                if let Some(last) = blocks.last_mut() {
                    last.lines.push(line.clone());
                }
                if is_display_math_delimiter(line) {
                    state = ParserState::Normal;
                }
            }
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 19 tests pass.

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser handles display math blocks ($$...$$)"
```

---

## Task 9: Parser — HTML tables

**Files:**
- Modify: `src/parser.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn html_table() {
        let input = "before\n<table>\n<tr><td>cell</td></tr>\n</table>\nafter";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::HtmlTable, 3),
            (BlockKind::Paragraph, 1),
        ]);
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
```

**Step 2: Run tests — they should fail**

**Step 3: Add `InHtmlTable` state and detection**

Add variant to `ParserState`:

```rust
    /// Inside an HTML `<table>...</table>` block.
    InHtmlTable,
```

Add helpers:

```rust
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
```

In `parse_blocks`, add HTML table detection in `Normal` arm (after display math, before `classify_line`):

```rust
                if is_html_table_open(line) {
                    // Check if it also closes on the same line
                    let closes = is_html_table_close(line);
                    blocks.push(Block {
                        kind: BlockKind::HtmlTable,
                        lines: vec![line.clone()],
                    });
                    if !closes {
                        state = ParserState::InHtmlTable;
                    }
                    continue;
                }
```

Add the match arm:

```rust
            ParserState::InHtmlTable => {
                if let Some(last) = blocks.last_mut() {
                    last.lines.push(line.clone());
                }
                if is_html_table_close(line) {
                    state = ParserState::Normal;
                }
            }
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 22 tests pass.

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser handles HTML tables (<table>...</table>)"
```

---

## Task 10: Parser — callout blocks and blockquotes

**Files:**
- Modify: `src/parser.rs`

Callouts are `> [!type]` lines (Obsidian syntax). Regular blockquotes are `> ` lines that are NOT callouts. A callout block continues as long as lines start with `>`.

**Step 1: Add failing tests**

```rust
    #[test]
    fn callout_block() {
        let input = "> [!info] Title\n> Content line\n> More content\n\nAfter";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![
            (BlockKind::Callout, 3),
            (BlockKind::BlankLine, 1),
            (BlockKind::Paragraph, 1),
        ]);
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
        assert_eq!(result, vec![
            (BlockKind::Callout, 2),
            (BlockKind::Paragraph, 1),
        ]);
    }

    #[test]
    fn nested_blockquote_in_callout() {
        // > [!info]
        // > > nested quote
        let input = "> [!info] Title\n> > Nested quote";
        let result = parse_to_kinds(input);
        assert_eq!(result, vec![(BlockKind::Callout, 2)]);
    }
```

**Step 2: Run tests — they should fail**

**Step 3: Add `InCallout` state and blockquote/callout detection**

Add to `ParserState`:

```rust
    /// Inside a callout block (`> [!type] ...`).
    InCallout,
```

Add helpers:

```rust
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
```

In `parse_blocks` Normal arm, add callout and blockquote detection (after HTML table, before `classify_line`):

```rust
                // Callout: > [!type]
                if is_callout_start(line) {
                    state = ParserState::InCallout;
                    blocks.push(Block {
                        kind: BlockKind::Callout,
                        lines: vec![line.clone()],
                    });
                    continue;
                }

                // Blockquote: > ... (not a callout)
                if is_blockquote_line(line) {
                    push_or_merge(&mut blocks, BlockKind::Blockquote, line.clone());
                    continue;
                }
```

Add the `InCallout` match arm:

```rust
            ParserState::InCallout => {
                if is_blockquote_line(line) {
                    // Continuation of the callout
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                } else {
                    // Callout ends — process this line in Normal state
                    state = ParserState::Normal;
                    let kind = classify_line(line);
                    push_or_merge(&mut blocks, kind, line.clone());
                }
            }
```

**Important:** The `InCallout` arm must also check for code fence opening (since the line is processed in Normal-like fashion if it's not a `>` line). But actually, since we're falling through to `classify_line` for non-`>` lines, code fences starting immediately after a callout won't be detected. Let's handle that:

Replace the `InCallout` else-branch with a re-dispatch to Normal logic:

```rust
            ParserState::InCallout => {
                if is_blockquote_line(line) {
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                } else {
                    // Callout ended. Re-classify this line in Normal state.
                    state = ParserState::Normal;

                    if let Some((fence_char, fence_count)) = detect_code_fence_open(line) {
                        state = ParserState::InCodeFence {
                            fence_char,
                            fence_count,
                        };
                        blocks.push(Block {
                            kind: BlockKind::CodeBlock,
                            lines: vec![line.clone()],
                        });
                    } else if is_display_math_delimiter(line) {
                        state = ParserState::InDisplayMath;
                        blocks.push(Block {
                            kind: BlockKind::DisplayMath,
                            lines: vec![line.clone()],
                        });
                    } else if is_html_table_open(line) {
                        let closes = is_html_table_close(line);
                        blocks.push(Block {
                            kind: BlockKind::HtmlTable,
                            lines: vec![line.clone()],
                        });
                        if !closes {
                            state = ParserState::InHtmlTable;
                        }
                    } else if is_callout_start(line) {
                        state = ParserState::InCallout;
                        blocks.push(Block {
                            kind: BlockKind::Callout,
                            lines: vec![line.clone()],
                        });
                    } else if is_blockquote_line(line) {
                        push_or_merge(&mut blocks, BlockKind::Blockquote, line.clone());
                    } else {
                        let kind = classify_line(line);
                        push_or_merge(&mut blocks, kind, line.clone());
                    }
                }
            }
```

**Wait — this duplicates the Normal arm logic.** That's a DRY violation. Instead, refactor: extract the Normal classification into a helper function that both `Normal` and `InCallout`'s else-branch can call. Here's the cleaner approach:

Create a helper method:

```rust
/// Process a single line under Normal-state classification rules.
///
/// Returns the new parser state (may transition to a multi-line block).
fn classify_normal_line(
    line: &str,
    blocks: &mut Vec<Block>,
) -> ParserState {
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

    if is_display_math_delimiter(line) {
        blocks.push(Block {
            kind: BlockKind::DisplayMath,
            lines: vec![line.to_string()],
        });
        return ParserState::InDisplayMath;
    }

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

    if is_callout_start(line) {
        blocks.push(Block {
            kind: BlockKind::Callout,
            lines: vec![line.to_string()],
        });
        return ParserState::InCallout;
    }

    if is_blockquote_line(line) {
        push_or_merge(blocks, BlockKind::Blockquote, line.to_string());
        return ParserState::Normal;
    }

    let kind = classify_line(line);
    push_or_merge(blocks, kind, line.to_string());
    ParserState::Normal
}
```

Then simplify both `Normal` and `InCallout` arms:

```rust
            ParserState::Normal => {
                state = classify_normal_line(line, &mut blocks);
            }
            // ... other arms ...
            ParserState::InCallout => {
                if is_blockquote_line(line) {
                    if let Some(last) = blocks.last_mut() {
                        last.lines.push(line.clone());
                    }
                } else {
                    state = classify_normal_line(line, &mut blocks);
                }
            }
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 28 tests pass.

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser handles callout blocks and blockquotes"
```

---

## Task 11: Parser — images, list items, and Markdown tables

**Files:**
- Modify: `src/parser.rs`

**Step 1: Add failing tests**

```rust
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
        // An image on its own line is an Image block
        let result = parse_to_kinds("text before\n![img](url)\ntext after");
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::Image, 1),
            (BlockKind::Paragraph, 1),
        ]);
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
        assert_eq!(result, vec![
            (BlockKind::Paragraph, 1),
            (BlockKind::BlankLine, 1),
            (BlockKind::MarkdownTable, 3),
            (BlockKind::BlankLine, 1),
            (BlockKind::Paragraph, 1),
        ]);
    }

    #[test]
    fn pipe_in_paragraph_not_table() {
        // A line with | but no separator row is just a paragraph
        let result = parse_to_kinds("value a | value b");
        assert_eq!(result, vec![(BlockKind::Paragraph, 1)]);
    }
```

**Step 2: Run tests — they should fail**

**Step 3: Add detection to `classify_line` and `classify_normal_line`**

Add these helpers:

```rust
/// Check if a line is an image (Obsidian `![[...]]` or standard `![...](...)` ).
fn is_image_line(line: &str) -> bool {
    let trimmed = line.trim();
    // Obsidian: ![[filename]]
    if trimmed.starts_with("![[") && trimmed.ends_with("]]") {
        return true;
    }
    // Standard markdown: ![alt](url)
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
    // Unordered: "- ", "* ", "+ "
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
/// Example: `|---|---|` or `| --- | :---: |`
fn is_table_separator(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.contains('|') || !trimmed.contains('-') {
        return false;
    }
    trimmed.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

/// Check if a line looks like a Markdown table row.
///
/// Must start and/or contain `|`.
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}
```

Now update `classify_normal_line` to add image, list, and table detection. Add these checks after the `is_blockquote_line` check, before the final `classify_line` fallback:

```rust
    if is_image_line(line) {
        push_or_merge(blocks, BlockKind::Image, line.to_string());
        return ParserState::Normal;
    }

    if is_list_item(line) {
        push_or_merge(blocks, BlockKind::ListItem, line.to_string());
        return ParserState::Normal;
    }
```

For Markdown tables, the detection is trickier because we need to see the separator row (line 2) to confirm it's a table. There are two approaches:

**Simple approach:** A line with `|` delimiters that follows another `|` line via `push_or_merge` is a potential table. We detect tables by checking if a `|`-line is being merged into a block whose previous line is a separator.

**Simpler approach (good enough):** Check if the line is a table separator row or a table row (starts and ends with `|`). If it's a separator, look back at the previous block — if it's a `Paragraph` whose lines are all `|`-rows, reclassify it as `MarkdownTable`. Otherwise, merge table rows and separators into `MarkdownTable` blocks.

Here's the practical approach. Add this logic to `classify_normal_line`:

```rust
    // Markdown table detection
    if is_table_separator(line) {
        // Check if previous block looks like a table header row
        if let Some(last) = blocks.last_mut() {
            if last.kind == BlockKind::Paragraph
                && last.lines.iter().all(|l| is_table_row(l))
            {
                // Reclassify previous paragraph as table, add separator
                last.kind = BlockKind::MarkdownTable;
                last.lines.push(line.to_string());
                return ParserState::Normal;
            }
        }
    }
    if is_table_row(line) {
        if let Some(last) = blocks.last() {
            if last.kind == BlockKind::MarkdownTable {
                // Rust note: need to re-borrow mutably since we borrowed
                // immutably above. Drop the immutable borrow first.
            }
        }
        if let Some(last) = blocks.last_mut() {
            if last.kind == BlockKind::MarkdownTable {
                last.lines.push(line.to_string());
                return ParserState::Normal;
            }
        }
    }
```

**Step 4: Run tests**

```bash
cargo test --lib parser
```

Expected: All 40 tests pass.

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/parser.rs
git commit -m "feat: parser detects images, list items, and Markdown tables"
```

---

## End of Part 2

At this point the parser handles all block types from the design:
- Blank lines, paragraphs, headings
- Fenced code blocks (3+ backticks/tildes, proper close matching)
- Display math (`$$...$$`)
- HTML tables (`<table>...</table>`)
- Callout blocks (`> [!type]`)
- Blockquotes (`>`)
- Images (Obsidian `![[...]]` and standard `![...](...)`)
- List items (unordered and ordered)
- Markdown tables (pipe-delimited with separator detection)

**Test count:** ~40 unit tests.

**Next:** Part 3 covers the extractor (pass 2) — bold/highlight span extraction from paragraph, blockquote, and list item blocks.
