# md-prune Implementation Plan — Part 3: Extractor (Pass 2)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the span extractor that finds `**bold**` and `==highlighted==` text within paragraphs, blockquotes, list items, and callout content.

**Prerequisite:** Part 2 complete (parser classifies all block types).

**Key file:** `src/extractor.rs`

---

## How the extractor works

The extractor operates on the **text content** of blocks classified as `Paragraph`, `Blockquote`, `ListItem`, or `Callout` (for callouts, it processes the text *after* stripping the `> ` prefix).

It uses `regex` to find bold and highlight spans, then constructs output lines containing only the matched spans (or the full paragraph, if `--keep-paragraph` is set).

**Key regex patterns:**
- Bold: `\*\*(.+?)\*\*` — matches `**...**` non-greedily
- Highlight: `==(.+?)==` — matches `==...==` non-greedily

**Why non-greedy (`+?`):** Given `**a** and **b**`, greedy would match `**a** and **b**` as one span. Non-greedy correctly finds two separate spans: `**a**` and `**b**`.

---

## Task 12: Extractor — basic bold extraction

**Files:**
- Create: `src/extractor.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Write failing tests**

Create `src/extractor.rs` with test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FilterMode;

    #[test]
    fn extract_single_bold_span() {
        let result = extract_marked_text("Hello **world** goodbye", FilterMode::Both, false);
        assert_eq!(result, Some("**world**".to_string()));
    }

    #[test]
    fn extract_multiple_bold_spans() {
        let result = extract_marked_text(
            "A **first** B **second** C",
            FilterMode::Both,
            false,
        );
        assert_eq!(result, Some("**first** **second**".to_string()));
    }

    #[test]
    fn no_bold_returns_none() {
        let result = extract_marked_text("No bold here", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn entire_line_is_bold() {
        let result = extract_marked_text(
            "**The whole line is bold.**",
            FilterMode::Both,
            false,
        );
        assert_eq!(result, Some("**The whole line is bold.**".to_string()));
    }

    #[test]
    fn bold_with_inline_code() {
        let result = extract_marked_text(
            "**Use `println!` for output.**",
            FilterMode::Both,
            false,
        );
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
        // ** without closing ** is not a bold span
        let result = extract_marked_text("Hello ** world", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn empty_bold_ignored() {
        // **** is empty bold — should not match
        let result = extract_marked_text("Before **** after", FilterMode::Both, false);
        assert_eq!(result, None);
    }
}
```

**Step 2: Run tests to verify they fail**

```bash
cargo test --lib extractor
```

Expected: Compilation error — `extract_marked_text` not defined.

**Step 3: Write the implementation**

Add above the test module in `src/extractor.rs`:

```rust
use regex::Regex;
use std::sync::LazyLock;

use crate::types::FilterMode;

// Rust note: `LazyLock` is like a global that's initialized once on first use.
// Compiling a Regex is expensive, so we do it once and reuse it.
// Python equivalent: module-level `re.compile(...)`.

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
/// # Examples
///
/// ```
/// # use md_prune::extractor::extract_marked_text;
/// # use md_prune::types::FilterMode;
/// let result = extract_marked_text("Hello **world**", FilterMode::Both, false);
/// assert_eq!(result, Some("**world**".to_string()));
/// ```
pub fn extract_marked_text(line: &str, mode: FilterMode, strip: bool) -> Option<String> {
    let mut spans: Vec<(usize, &str)> = Vec::new();

    // Collect bold spans
    if matches!(mode, FilterMode::Both | FilterMode::BoldOnly) {
        for m in BOLD_RE.find_iter(line) {
            spans.push((m.start(), m.as_str()));
        }
    }

    // Collect highlight spans
    if matches!(mode, FilterMode::Both | FilterMode::HighlightOnly) {
        for m in HIGHLIGHT_RE.find_iter(line) {
            spans.push((m.start(), m.as_str()));
        }
    }

    if spans.is_empty() {
        return None;
    }

    // Sort by position so spans appear in document order.
    // Deduplicate overlapping spans (e.g. bold containing highlight).
    spans.sort_by_key(|(pos, _)| *pos);
    spans.dedup_by(|a, b| {
        // If b starts at or before a ends, they overlap — keep the longer one
        let b_end = b.0 + b.1.len();
        let a_end = a.0 + a.1.len();
        if a.0 <= b.0 && a_end >= b_end {
            // a contains b — drop b (the duplicate)
            true
        } else if b.0 <= a.0 && b_end >= a_end {
            // b contains a — keep b, replace a with b's data
            *a = *b;
            true
        } else {
            false
        }
    });

    let extracted: Vec<&str> = spans.iter().map(|(_, s)| *s).collect();
    let joined = extracted.join(" ");

    let result = if strip { strip_markers(&joined) } else { joined };

    Some(result)
}

/// Remove `**` and `==` delimiters from text.
///
/// Applied as a post-processing step when `--strip-markers` is set.
fn strip_markers(text: &str) -> String {
    text.replace("**", "").replace("==", "")
}
```

**Step 4: Register the module**

Add `mod extractor;` to `src/main.rs`:

```rust
mod cli;
mod errors;
mod extractor;
mod parser;
mod types;
```

**Step 5: Run tests**

```bash
cargo test --lib extractor
```

Expected: All 8 tests pass.

**Step 6: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs src/main.rs
git commit -m "feat: basic bold span extraction with regex"
```

---

## Task 13: Extractor — highlight extraction

**Files:**
- Modify: `src/extractor.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn extract_highlight_span() {
        let result = extract_marked_text("Hello ==world== goodbye", FilterMode::Both, false);
        assert_eq!(result, Some("==world==".to_string()));
    }

    #[test]
    fn extract_multiple_highlights() {
        let result = extract_marked_text(
            "A ==first== B ==second== C",
            FilterMode::Both,
            false,
        );
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
        let result = extract_marked_text(
            "**bold** and ==highlight==",
            FilterMode::BoldOnly,
            false,
        );
        assert_eq!(result, Some("**bold**".to_string()));
    }

    #[test]
    fn unclosed_highlight_is_literal() {
        let result = extract_marked_text("Hello == world", FilterMode::Both, false);
        assert_eq!(result, None);
    }
```

**Step 2: Run tests**

```bash
cargo test --lib extractor
```

Expected: These should already pass because the implementation in Task 12 handles both bold and highlight. If any fail, debug.

**Step 3: Pre-flight checks and commit (if tests all pass)**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "test: add highlight extraction tests (implementation already supports it)"
```

---

## Task 14: Extractor — nested bold and highlight

This handles the tricky case from the reference files: `**==text==**` and `**text ==inner== more**`.

**Files:**
- Modify: `src/extractor.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn nested_highlight_inside_bold() {
        // **==text==** — bold wraps highlight
        let result = extract_marked_text(
            "**==For every successful language==**",
            FilterMode::Both,
            false,
        );
        // Both markers should be present, as one span
        assert_eq!(
            result,
            Some("**==For every successful language==**".to_string())
        );
    }

    #[test]
    fn nested_highlight_inside_bold_with_extra() {
        // **==highlight== more bold text**
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
        // In bold-only mode, **==text==** matches the bold wrapper
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
        // In highlight-only mode, **==text==** extracts only ==text==
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
        // From references/example01.md line 26
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
```

**Step 2: Run tests**

```bash
cargo test --lib extractor
```

Check which tests pass/fail. The dedup logic from Task 12 should handle overlapping spans. If tests fail, the overlap dedup needs adjustment.

**Step 3: Fix if needed, then commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "test: add nested bold/highlight extraction tests"
```

---

## Task 15: Extractor — strip markers

**Files:**
- Modify: `src/extractor.rs`

**Step 1: Add failing tests**

```rust
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
        let result = extract_marked_text(
            "**bold** and ==highlight==",
            FilterMode::Both,
            true,
        );
        assert_eq!(result, Some("bold highlight".to_string()));
    }
```

**Step 2: Run tests — these should already pass**

The `strip_markers` function and the `strip` parameter were already implemented in Task 12.

```bash
cargo test --lib extractor
```

Expected: All pass.

**Step 3: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "test: add strip-markers tests (implementation already supports it)"
```

---

## Task 16: Extractor — escaped markers

**Files:**
- Modify: `src/extractor.rs`

Escaped markers (`\*\*`, `\=\=`) should be treated as literal text, not as bold/highlight delimiters.

**Step 1: Add failing tests**

```rust
    #[test]
    fn escaped_bold_is_literal() {
        let result = extract_marked_text(r"Hello \*\*not bold\*\* bye", FilterMode::Both, false);
        assert_eq!(result, None);
    }

    #[test]
    fn escaped_highlight_is_literal() {
        let result = extract_marked_text(r"Hello \=\=not highlight\=\= bye", FilterMode::Both, false);
        assert_eq!(result, None);
    }
```

**Step 2: Run tests — check if they pass**

```bash
cargo test --lib extractor
```

The regex `\*\*(.+?)\*\*` uses literal `\*` in the regex, which matches the `*` character. The escaped `\*\*` in the source text is the literal string `\*\*` — two characters `\` and `*`. Since the regex only matches `**` (two asterisks with nothing in between), the `\*\*` in the input won't match. However, we need to verify.

If the backslash-escaped forms `\*\*` do NOT match the regex (they shouldn't because `\*` is `\` then `*`, not `**`), these tests pass automatically.

**Step 3: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "test: verify escaped markers are treated as literal text"
```

---

## Task 17: Extractor — process block content

Now we need a higher-level function that processes an entire `Block`'s lines and returns the extracted output lines.

**Files:**
- Modify: `src/extractor.rs`

**Step 1: Add failing tests**

```rust
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
        // All lines kept because at least one has bold
        assert_eq!(result, vec!["Normal text.", "**Bold text.** More normal.", "Also normal."]);
    }

    #[test]
    fn extract_from_paragraph_no_bold() {
        let lines = vec![
            "No bold here.".to_string(),
            "None here either.".to_string(),
        ];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert!(result.is_empty());
    }

    #[test]
    fn extract_from_list_item() {
        let lines = vec!["- **For every output tolerance $\\varepsilon$, there is some input tolerance $\\delta$ that works.**".to_string()];
        let result = extract_from_lines(&lines, FilterMode::Both, false, false);
        assert_eq!(result, vec!["- **For every output tolerance $\\varepsilon$, there is some input tolerance $\\delta$ that works.**"]);
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
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib extractor
```

Expected: `extract_from_lines` not defined.

**Step 3: Implement `extract_from_lines`**

```rust
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
        // Check if any line has marked text
        let has_any = lines
            .iter()
            .any(|line| {
                let content = strip_line_prefix(line);
                extract_marked_text(content, mode, false).is_some()
            });
        if has_any {
            return lines.to_vec();
        }
        return Vec::new();
    }

    // Default mode: extract only marked spans from each line
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

/// Strip blockquote `> ` or list item `- ` prefixes to get the content.
///
/// Used when checking for bold/highlight spans — the prefix isn't part
/// of the content we're scanning.
fn strip_line_prefix(line: &str) -> &str {
    // Blockquote prefix: "> " or ">"
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
///
/// # Examples
///
/// - `"> **bold**"` → `("> ", "**bold**")`
/// - `"- **item**"` → `("- ", "**item**")`
/// - `"plain"` → `("", "plain")`
fn split_line_prefix(line: &str) -> (&str, &str) {
    // Blockquote prefix
    if line.starts_with("> ") {
        return (&line[..2], &line[2..]);
    }
    if line.starts_with('>') {
        return (&line[..1], &line[1..]);
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
```

**Step 4: Run tests**

```bash
cargo test --lib extractor
```

Expected: All tests pass (previous + new).

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "feat: extract_from_lines processes block content with prefix handling"
```

---

## Task 18: Extractor — callout content extraction

Callouts need special handling: we strip the `> ` prefix for extraction, but preserve the callout structure in output.

**Files:**
- Modify: `src/extractor.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn extract_from_callout_lines() {
        let lines = vec![
            "> [!info] Curry-Howard isomorphism".to_string(),
            "> Normal callout text.".to_string(),
            "> **A logical statement** is a kind of **specification**".to_string(),
        ];
        let result = extract_from_callout(&lines, FilterMode::Both, false, false);
        // Keep the callout header, then only lines with bold/highlight
        assert_eq!(result, vec![
            "> [!info] Curry-Howard isomorphism",
            "> **A logical statement** is a kind of **specification**",
        ]);
    }

    #[test]
    fn extract_from_callout_no_bold() {
        let lines = vec![
            "> [!info] Title".to_string(),
            "> No bold here.".to_string(),
        ];
        let result = extract_from_callout(&lines, FilterMode::Both, false, false);
        // No bold content — return empty (callout is dropped)
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
        // keep_paragraph: all lines kept because at least one has bold
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
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib extractor
```

Expected: `extract_from_callout` not defined.

**Step 3: Implement**

```rust
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

    let mut output = Vec::new();
    for line in lines {
        let content = strip_line_prefix(line);
        if extract_marked_text(content, mode, false).is_some() {
            if strip {
                let extracted = extract_marked_text(content, mode, true)
                    .expect("already confirmed match exists");
                // Reconstruct with prefix
                let prefix_len = line.len() - content.len();
                output.push(format!("{}{}", &line[..prefix_len], extracted));
            } else {
                output.push(line.clone());
            }
        }
    }
    output
}
```

**Step 4: Run tests**

```bash
cargo test --lib extractor
```

Expected: All tests pass.

**Step 5: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/extractor.rs
git commit -m "feat: callout content extraction preserving > [!type] structure"
```

---

## End of Part 3

The extractor now handles:
- Bold span extraction (`**...**`)
- Highlight span extraction (`==...==`)
- Nested bold/highlight spans (dedup overlaps)
- Filter modes: BoldOnly, HighlightOnly, Both
- Strip markers option
- Line prefix handling (blockquote `>`, list items `- `, `1. `)
- Callout-specific extraction (preserves `> [!type]` header)
- keep-paragraph mode (all or nothing)
- Escaped markers treated as literal

**Test count:** ~30+ extractor unit tests.

**Next:** Part 4 covers the pruner (orchestration + heading pruning) and wiring it all into `main.rs`.
