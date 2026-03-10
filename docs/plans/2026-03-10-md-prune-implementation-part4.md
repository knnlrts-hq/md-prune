# md-prune Implementation Plan — Part 4: Pruner + Main Wiring

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the orchestration layer that combines parsing and extraction, handles heading pruning, and wire everything into a working CLI binary.

**Prerequisite:** Parts 1-3 complete (types, errors, CLI, parser, extractor all tested).

**Key files:** `src/pruner.rs`, `src/main.rs`

---

## How the pruner works

The pruner is the high-level orchestrator:

1. Read input file → `Vec<String>` of lines
2. Call `parse_blocks()` → `Vec<Block>`
3. For each block, decide what to do based on its `BlockKind` and the `Config`:
   - **Heading:** Keep if `config.keep_headings` (tentatively — may prune later)
   - **CodeBlock:** Keep if `config.keep_code_blocks`, emit all lines verbatim
   - **DisplayMath:** Keep if `config.keep_math`, emit all lines verbatim
   - **Callout:** Keep if `config.keep_callouts`, extract bold/highlight within
   - **HtmlTable / MarkdownTable:** Keep if `config.keep_tables`, emit all lines verbatim
   - **Image:** Keep if `config.keep_images`, emit the line verbatim
   - **Paragraph / Blockquote / ListItem:** Run extractor, emit only matched spans
   - **BlankLine:** Emit between content blocks for readability
4. After generating output blocks, prune headings (unless `--keep-all-headings`):
   - A heading is pruned if no content appears between it and the next heading of equal/higher level
5. Clean up: remove trailing blank lines, collapse multiple consecutive blank lines

---

## Task 19: Pruner — basic block filtering

**Files:**
- Create: `src/pruner.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Write failing tests**

Create `src/pruner.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Config;

    #[test]
    fn prune_simple_bold_paragraph() {
        let input = "Hello **world** goodbye";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "**world**\n");
    }

    #[test]
    fn prune_no_bold_returns_empty_string() {
        let input = "No bold text anywhere.";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_heading_with_bold_beneath() {
        let input = "# Title\n**Bold paragraph.**";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "# Title\n**Bold paragraph.**\n");
    }

    #[test]
    fn prune_heading_without_bold_beneath_is_dropped() {
        let input = "# Title\nNo bold here.";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_no_headings_flag() {
        let input = "# Title\n**Bold text.**";
        let config = Config {
            keep_headings: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "**Bold text.**\n");
    }

    #[test]
    fn prune_code_block_kept_by_default() {
        let input = "```rust\nfn main() {}\n```";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "```rust\nfn main() {}\n```\n");
    }

    #[test]
    fn prune_code_block_excluded() {
        let input = "```rust\nfn main() {}\n```";
        let config = Config {
            keep_code_blocks: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_image_kept_by_default() {
        let input = "![[my-image.png]]";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "![[my-image.png]]\n");
    }

    #[test]
    fn prune_image_excluded() {
        let input = "![[my-image.png]]";
        let config = Config {
            keep_images: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_display_math_kept_by_default() {
        let input = "$$\nx^2 + 1\n$$";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "$$\nx^2 + 1\n$$\n");
    }

    #[test]
    fn prune_display_math_excluded() {
        let input = "$$\nx^2 + 1\n$$";
        let config = Config {
            keep_math: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_html_table_kept_by_default() {
        let input = "<table>\n<tr><td>cell</td></tr>\n</table>";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "<table>\n<tr><td>cell</td></tr>\n</table>\n");
    }

    #[test]
    fn prune_html_table_excluded() {
        let input = "<table>\n<tr><td>cell</td></tr>\n</table>";
        let config = Config {
            keep_tables: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "");
    }
}
```

**Step 2: Run tests — they should fail**

```bash
cargo test --lib pruner
```

Expected: `prune` not defined.

**Step 3: Implement the pruner**

```rust
use crate::extractor::{extract_from_callout, extract_from_lines};
use crate::parser::parse_blocks;
use crate::types::{BlockKind, Config};

/// Prune a Markdown document, keeping only bold/highlighted text and
/// optionally retained structural elements.
///
/// This is the main entry point for the pruning engine. It orchestrates
/// the two-pass pipeline: parse blocks → extract content → prune headings.
///
/// # Arguments
///
/// * `input` - The raw Markdown source text
/// * `config` - Runtime configuration from CLI flags
///
/// # Returns
///
/// The pruned Markdown as a `String`.
pub fn prune(input: &str, config: &Config) -> String {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let blocks = parse_blocks(&lines);

    // Phase 1: Generate output lines for each block
    // Each element is (BlockKind, Vec<output_lines>)
    let mut output_blocks: Vec<(BlockKind, Vec<String>)> = Vec::new();

    for block in &blocks {
        let output_lines = match &block.kind {
            BlockKind::BlankLine => Vec::new(), // Handle spacing later
            BlockKind::Heading { .. } => {
                if config.keep_headings {
                    block.lines.clone()
                } else {
                    Vec::new()
                }
            }
            BlockKind::CodeBlock => {
                if config.keep_code_blocks {
                    block.lines.clone()
                } else {
                    Vec::new()
                }
            }
            BlockKind::DisplayMath => {
                if config.keep_math {
                    block.lines.clone()
                } else {
                    Vec::new()
                }
            }
            BlockKind::HtmlTable | BlockKind::MarkdownTable => {
                if config.keep_tables {
                    block.lines.clone()
                } else {
                    Vec::new()
                }
            }
            BlockKind::Image => {
                if config.keep_images {
                    block.lines.clone()
                } else {
                    Vec::new()
                }
            }
            BlockKind::Callout => {
                if config.keep_callouts {
                    extract_from_callout(
                        &block.lines,
                        config.filter_mode,
                        config.strip_markers,
                        config.keep_paragraph,
                    )
                } else {
                    Vec::new()
                }
            }
            BlockKind::Paragraph | BlockKind::Blockquote | BlockKind::ListItem => {
                extract_from_lines(
                    &block.lines,
                    config.filter_mode,
                    config.strip_markers,
                    config.keep_paragraph,
                )
            }
        };

        if !output_lines.is_empty() {
            output_blocks.push((block.kind.clone(), output_lines));
        }
    }

    // Phase 2: Prune empty headings (unless --keep-all-headings)
    if config.keep_headings && !config.keep_all_headings {
        prune_empty_headings(&mut output_blocks);
    }

    // Phase 3: Render output with blank line separation
    render_output(&output_blocks)
}

/// Remove headings that have no content beneath them.
///
/// A heading is "empty" if the next item in output_blocks is another
/// heading of equal or higher level (lower number), or if there are
/// no more items after it.
fn prune_empty_headings(blocks: &mut Vec<(BlockKind, Vec<String>)>) {
    // Work backwards so removals don't shift indices we haven't processed
    let mut i = blocks.len();
    while i > 0 {
        i -= 1;
        if let BlockKind::Heading { level } = &blocks[i].0 {
            let level = *level;
            // Check if any non-heading content follows before the next
            // heading of equal or higher level
            let has_content = blocks[i + 1..]
                .iter()
                .take_while(|(kind, _)| {
                    // Stop at equal or higher level heading
                    if let BlockKind::Heading { level: next_level } = kind {
                        *next_level > level // Lower level headings don't stop us
                    } else {
                        true
                    }
                })
                .any(|(kind, _)| !matches!(kind, BlockKind::Heading { .. }));

            if !has_content {
                blocks.remove(i);
            }
        }
    }
}

/// Join output blocks into a single string with blank lines between blocks.
fn render_output(blocks: &[(BlockKind, Vec<String>)]) -> String {
    if blocks.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for (i, (_, lines)) in blocks.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        for line in lines {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}
```

**Step 4: Register the module**

Add to `src/main.rs`:

```rust
mod cli;
mod errors;
mod extractor;
mod parser;
mod pruner;
mod types;
```

**Step 5: Run tests**

```bash
cargo test --lib pruner
```

Expected: All 14 tests pass.

**Step 6: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/pruner.rs src/main.rs
git commit -m "feat: pruner orchestrates parsing, extraction, and heading pruning"
```

---

## Task 20: Pruner — heading pruning edge cases

**Files:**
- Modify: `src/pruner.rs`

**Step 1: Add failing tests**

```rust
    #[test]
    fn prune_nested_headings_with_deep_content() {
        let input = "# Top\n## Empty Section\n## Section With Content\n**Bold here.**";
        let config = Config::default();
        let result = prune(input, &config);
        // # Top kept (has content below eventually)
        // ## Empty Section dropped (no content before next ##)
        // ## Section With Content kept
        assert_eq!(result, "# Top\n\n## Section With Content\n**Bold here.**\n");
    }

    #[test]
    fn prune_keep_all_headings() {
        let input = "# Title\n## Empty\nNo bold.";
        let config = Config {
            keep_all_headings: true,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "# Title\n\n## Empty\n");
    }

    #[test]
    fn prune_heading_chain_all_empty() {
        let input = "# A\n## B\n### C\nNo bold text.";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_multiple_sections_mixed() {
        let input = "# A\n**Bold A.**\n# B\nNo bold.\n# C\n**Bold C.**";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "# A\n**Bold A.**\n\n# C\n**Bold C.**\n");
    }

    #[test]
    fn prune_callout_with_bold_kept() {
        let input = "> [!info] Title\n> **Bold in callout.**\n> Normal text.";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "> [!info] Title\n> **Bold in callout.**\n");
    }

    #[test]
    fn prune_callout_excluded() {
        let input = "> [!info] Title\n> **Bold in callout.**";
        let config = Config {
            keep_callouts: false,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "");
    }

    #[test]
    fn prune_list_with_bold_items() {
        let input = "- Normal item.\n- **Bold item.**\n- Another normal.";
        let config = Config::default();
        let result = prune(input, &config);
        assert_eq!(result, "- **Bold item.**\n");
    }

    #[test]
    fn prune_keep_paragraph_mode() {
        let input = "Start. **Bold.** End.";
        let config = Config {
            keep_paragraph: true,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "Start. **Bold.** End.\n");
    }

    #[test]
    fn prune_strip_markers() {
        let input = "Hello **world** and ==highlight==.";
        let config = Config {
            strip_markers: true,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "world highlight\n");
    }

    #[test]
    fn prune_bold_only_mode() {
        let input = "**Bold** and ==highlight==.";
        let config = Config {
            filter_mode: crate::types::FilterMode::BoldOnly,
            ..Config::default()
        };
        let result = prune(input, &config);
        assert_eq!(result, "**Bold**\n");
    }

    #[test]
    fn prune_empty_input() {
        let config = Config::default();
        let result = prune("", &config);
        assert_eq!(result, "");
    }
```

**Step 2: Run tests and fix any failures**

```bash
cargo test --lib pruner
```

Debug and fix any failing tests. Pay particular attention to:
- Blank line insertion between blocks
- Heading pruning order (backwards traversal)
- Edge cases with the callout extractor returning the header line

**Step 3: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/pruner.rs
git commit -m "test: heading pruning edge cases and feature flag combinations"
```

---

## Task 21: Wire everything into `main.rs`

**Files:**
- Modify: `src/main.rs`

**Step 1: Write the full main function**

Replace `src/main.rs`:

```rust
mod cli;
mod errors;
mod extractor;
mod parser;
mod pruner;
mod types;

use std::fs;
use std::io::Write;

use anyhow::Context;
use clap::Parser;

use crate::cli::Cli;
use crate::errors::MdPruneError;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = cli.to_config();

    // Read input file
    let input_path = &cli.input;
    if !input_path.exists() {
        return Err(MdPruneError::FileNotFound {
            path: input_path.clone(),
        }
        .into());
    }

    let input = fs::read_to_string(input_path).map_err(|e| MdPruneError::ReadError {
        path: input_path.clone(),
        source: e,
    })?;

    // Handle UTF-8 BOM
    let input = input.strip_prefix('\u{feff}').unwrap_or(&input);

    // Run the pruner
    let output = pruner::prune(input, &config);

    // Warn if no content was extracted
    if output.is_empty() {
        eprintln!(
            "md-prune: warning: no bold or highlighted text found in {:?}",
            input_path
        );
    }

    // Write output
    match &cli.output {
        Some(output_path) => {
            fs::write(output_path, &output).map_err(|e| MdPruneError::WriteError {
                path: output_path.clone(),
                source: e,
            })?;
            eprintln!("md-prune: wrote output to {:?}", output_path);
        }
        None => {
            // Rust note: stdout().write_all is more efficient than print!
            // for large outputs because it avoids formatting overhead.
            std::io::stdout()
                .write_all(output.as_bytes())
                .context("failed to write to stdout")?;
        }
    }

    Ok(())
}
```

**Step 2: Smoke test with reference files**

```bash
cargo run -- references/example01.md
cargo run -- references/example02.md
cargo run -- --bold-only references/example01.md
cargo run -- --highlight-only references/example01.md
cargo run -- --no-headings references/example01.md
cargo run -- --no-code-blocks --no-images references/example01.md
cargo run -- --strip-markers references/example02.md
cargo run -- -o /tmp/test-output.md references/example01.md && cat /tmp/test-output.md
```

Verify each command produces reasonable output. Look for:
- Bold text is extracted from example01.md and example02.md
- `--bold-only` drops `==highlight==` text
- `--highlight-only` extracts only `==...==` spans
- `--no-headings` removes section headers
- `--no-code-blocks --no-images` drops code blocks and images
- `-o` writes to file

**Step 3: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/main.rs
git commit -m "feat: wire pruner into main with file I/O and error handling"
```

---

## End of Part 4

The tool is now fully functional end-to-end:
- CLI parses all flags correctly
- Parser classifies blocks
- Extractor finds bold/highlight spans
- Pruner orchestrates everything and prunes empty headings
- Main reads files, writes output, handles errors

**Next:** Part 5 covers integration tests with the reference files, golden file snapshots, and the README.
