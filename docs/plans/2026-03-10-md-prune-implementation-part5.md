# md-prune Implementation Plan — Part 5: Integration Tests + README

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add integration tests using the reference files, create golden file snapshots, and write the README.

**Prerequisite:** Parts 1-4 complete (tool is fully functional end-to-end).

**Key files:** `tests/integration.rs`, `tests/expected/`, `README.md`

---

## Task 22: Generate golden files

Golden files are the expected output for each reference file + flag combination. Generate them by running the tool and reviewing the output manually.

**Files:**
- Create: `tests/expected/` directory
- Create: golden files for each test case

**Step 1: Create the expected output directory**

```bash
mkdir -p tests/expected
```

**Step 2: Generate baseline golden files**

Run the tool on each reference file with default flags and save output:

```bash
cargo run -- references/example01.md > tests/expected/example01_default.md
cargo run -- references/example02.md > tests/expected/example02_default.md
cargo run -- references/example03.md > tests/expected/example03_default.md
```

**Step 3: Review each golden file**

**Critically important:** Open each golden file and verify:

1. **All bold text is present.** Search the reference file for `**` and confirm each bold span appears in the golden file.
2. **All highlight text is present.** Search for `==` in the reference files and confirm.
3. **No unmarked text leaked through.** Paragraphs without bold/highlight should NOT appear.
4. **Structural elements are present:**
   - Headings (only those with content beneath them)
   - Images (`![[...]]` and `![...](...)`)
   - Code blocks (fenced with backticks)
   - Display math (`$$...$$`)
   - HTML tables (`<table>...</table>`)
   - Callout blocks (`> [!type]`)
5. **Empty headings are pruned.** Sections with no bold/highlighted content should not have their heading in the output.

If any output is wrong, debug the parser/extractor/pruner and fix before proceeding.

**Step 4: Generate flag-specific golden files**

```bash
cargo run -- --bold-only references/example01.md > tests/expected/example01_bold_only.md
cargo run -- --highlight-only references/example01.md > tests/expected/example01_highlight_only.md
cargo run -- --no-headings references/example01.md > tests/expected/example01_no_headings.md
cargo run -- --no-images references/example01.md > tests/expected/example01_no_images.md
cargo run -- --no-code-blocks references/example01.md > tests/expected/example01_no_code_blocks.md
cargo run -- --no-tables references/example03.md > tests/expected/example03_no_tables.md
cargo run -- --no-math references/example02.md > tests/expected/example02_no_math.md
cargo run -- --no-callouts references/example01.md > tests/expected/example01_no_callouts.md
cargo run -- --strip-markers references/example02.md > tests/expected/example02_strip_markers.md
cargo run -- --keep-paragraph references/example02.md > tests/expected/example02_keep_paragraph.md
cargo run -- --keep-all-headings references/example01.md > tests/expected/example01_keep_all_headings.md
```

**Review each one carefully** using the same checklist as Step 3.

**Step 5: Commit golden files**

```bash
git add tests/expected/
git commit -m "test: add golden file expected outputs for integration tests"
```

---

## Task 23: Write integration tests

**Files:**
- Create: `tests/integration.rs`

Integration tests in Rust go in the `tests/` directory. Each file is compiled as a separate crate that can only access `pub` items from your library. Since our code is a binary (not a library), we need a slightly different approach.

**Step 1: Create a library root for testability**

Create `src/lib.rs` to re-export the pruning engine for integration tests:

```rust
pub mod cli;
pub mod errors;
pub mod extractor;
pub mod parser;
pub mod pruner;
pub mod types;
```

Then update `src/main.rs` to use the library:

```rust
use clap::Parser;
use std::fs;
use std::io::Write;

use anyhow::Context;
use md_prune::cli::Cli;
use md_prune::errors::MdPruneError;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = cli.to_config();

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

    let input = input.strip_prefix('\u{feff}').unwrap_or(&input);
    let output = md_prune::pruner::prune(input, &config);

    if output.is_empty() {
        eprintln!(
            "md-prune: warning: no bold or highlighted text found in {:?}",
            input_path
        );
    }

    match &cli.output {
        Some(output_path) => {
            fs::write(output_path, &output).map_err(|e| MdPruneError::WriteError {
                path: output_path.clone(),
                source: e,
            })?;
            eprintln!("md-prune: wrote output to {:?}", output_path);
        }
        None => {
            std::io::stdout()
                .write_all(output.as_bytes())
                .context("failed to write to stdout")?;
        }
    }

    Ok(())
}
```

Remove the `mod` declarations from `main.rs` since they're now in `lib.rs`.

**Step 2: Write integration tests**

Create `tests/integration.rs`:

```rust
use std::fs;

use md_prune::pruner::prune;
use md_prune::types::{Config, FilterMode};

/// Helper: read a file, prune with given config, compare to expected output.
fn assert_prune_matches(input_path: &str, expected_path: &str, config: &Config) {
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to read input {}: {}", input_path, e));
    let expected = fs::read_to_string(expected_path)
        .unwrap_or_else(|e| panic!("Failed to read expected {}: {}", expected_path, e));

    let actual = prune(&input, config);

    if actual != expected {
        // Show a useful diff on failure
        let actual_lines: Vec<&str> = actual.lines().collect();
        let expected_lines: Vec<&str> = expected.lines().collect();

        let max_lines = actual_lines.len().max(expected_lines.len());
        for i in 0..max_lines {
            let a = actual_lines.get(i).unwrap_or(&"<missing>");
            let e = expected_lines.get(i).unwrap_or(&"<missing>");
            if a != e {
                panic!(
                    "Mismatch at line {} (0-indexed) for {}:\n  expected: {:?}\n  actual:   {:?}\n\nFull actual output:\n{}",
                    i, input_path, e, a, actual
                );
            }
        }

        // If we get here, line counts differ
        panic!(
            "Output line count mismatch for {}: expected {} lines, got {}",
            input_path,
            expected_lines.len(),
            actual_lines.len(),
        );
    }
}

// ---- Default flags ----

#[test]
fn example01_default() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_default.md",
        &Config::default(),
    );
}

#[test]
fn example02_default() {
    assert_prune_matches(
        "references/example02.md",
        "tests/expected/example02_default.md",
        &Config::default(),
    );
}

#[test]
fn example03_default() {
    assert_prune_matches(
        "references/example03.md",
        "tests/expected/example03_default.md",
        &Config::default(),
    );
}

// ---- Filter modes ----

#[test]
fn example01_bold_only() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_bold_only.md",
        &Config {
            filter_mode: FilterMode::BoldOnly,
            ..Config::default()
        },
    );
}

#[test]
fn example01_highlight_only() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_highlight_only.md",
        &Config {
            filter_mode: FilterMode::HighlightOnly,
            ..Config::default()
        },
    );
}

// ---- Content toggle flags ----

#[test]
fn example01_no_headings() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_no_headings.md",
        &Config {
            keep_headings: false,
            ..Config::default()
        },
    );
}

#[test]
fn example01_no_images() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_no_images.md",
        &Config {
            keep_images: false,
            ..Config::default()
        },
    );
}

#[test]
fn example01_no_code_blocks() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_no_code_blocks.md",
        &Config {
            keep_code_blocks: false,
            ..Config::default()
        },
    );
}

#[test]
fn example03_no_tables() {
    assert_prune_matches(
        "references/example03.md",
        "tests/expected/example03_no_tables.md",
        &Config {
            keep_tables: false,
            ..Config::default()
        },
    );
}

#[test]
fn example02_no_math() {
    assert_prune_matches(
        "references/example02.md",
        "tests/expected/example02_no_math.md",
        &Config {
            keep_math: false,
            ..Config::default()
        },
    );
}

#[test]
fn example01_no_callouts() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_no_callouts.md",
        &Config {
            keep_callouts: false,
            ..Config::default()
        },
    );
}

// ---- Extraction behavior flags ----

#[test]
fn example02_strip_markers() {
    assert_prune_matches(
        "references/example02.md",
        "tests/expected/example02_strip_markers.md",
        &Config {
            strip_markers: true,
            ..Config::default()
        },
    );
}

#[test]
fn example02_keep_paragraph() {
    assert_prune_matches(
        "references/example02.md",
        "tests/expected/example02_keep_paragraph.md",
        &Config {
            keep_paragraph: true,
            ..Config::default()
        },
    );
}

#[test]
fn example01_keep_all_headings() {
    assert_prune_matches(
        "references/example01.md",
        "tests/expected/example01_keep_all_headings.md",
        &Config {
            keep_all_headings: true,
            ..Config::default()
        },
    );
}
```

**Step 3: Run integration tests**

```bash
cargo test --test integration
```

Expected: All tests pass (they compare against the golden files you just generated and reviewed).

**Step 4: Pre-flight checks and commit**

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo build && cargo test
git add src/lib.rs src/main.rs tests/integration.rs
git commit -m "feat: add integration tests with golden file snapshots"
```

---

## Task 24: Write the README

**Files:**
- Create: `README.md`

**Step 1: Write the README**

```markdown
# md-prune

Extract bold (`**...**`) and highlighted (`==...==`) text from Markdown files,
producing a pruned document that keeps only the key passages you've marked.

## Use Cases

- **Study note distillation**: Extract key takeaways from textbook notes
  you've bolded/highlighted while reading
- **Document summarization**: Reduce long Markdown documents to their
  most important points

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) 1.70+.

```bash
git clone https://github.com/your-org/md-prune.git
cd md-prune
cargo build --release
```

The binary will be at `target/release/md-prune`.

#### Platform notes

- **Linux**: No additional dependencies. Tested on Ubuntu 22.04+, Fedora 38+.
- **macOS**: No additional dependencies. Tested on macOS 13+ (Intel and Apple Silicon).
- **Windows**: No additional dependencies. Tested on Windows 10+ with MSVC toolchain.
  Use `cargo build --release` from a Developer Command Prompt or PowerShell.

## Usage

```
md-prune [OPTIONS] <INPUT>
```

### Basic examples

```bash
# Print pruned output to stdout
md-prune notes.md

# Save to a file
md-prune notes.md -o pruned.md

# Pipe to another tool
md-prune notes.md | less
```

### Filter modes

By default, both bold and highlighted text are extracted.

```bash
# Only bold text
md-prune --bold-only notes.md

# Only highlighted text
md-prune --highlight-only notes.md
```

### Excluding content types

By default, headings, images, tables, code blocks, math blocks, and callouts
are all kept in the output. Use `--no-*` flags to exclude them:

```bash
# Minimal output: just the bold/highlighted text
md-prune --no-headings --no-images --no-tables --no-code-blocks --no-math --no-callouts notes.md

# Keep headings but drop everything else
md-prune --no-images --no-tables --no-code-blocks --no-math --no-callouts notes.md
```

### Extraction behavior

```bash
# Keep entire paragraph if any part is bold/highlighted
md-prune --keep-paragraph notes.md

# Keep all headings, even sections with no bold/highlighted text
md-prune --keep-all-headings notes.md

# Strip the ** and == markers from the output
md-prune --strip-markers notes.md
```

### All options

| Flag | Default | Description |
|---|---|---|
| `-o, --output <FILE>` | stdout | Output file path |
| `--bold-only` | off | Only extract `**bold**` text |
| `--highlight-only` | off | Only extract `==highlighted==` text |
| `--no-headings` | off | Exclude headings |
| `--no-images` | off | Exclude images |
| `--no-tables` | off | Exclude tables (Markdown and HTML) |
| `--no-code-blocks` | off | Exclude fenced code blocks |
| `--no-math` | off | Exclude display math blocks |
| `--no-callouts` | off | Exclude callout blocks |
| `--keep-paragraph` | off | Keep full paragraph if any part is marked |
| `--keep-all-headings` | off | Keep all headings, even empty sections |
| `--strip-markers` | off | Remove `**` and `==` from output |

## Supported Markdown features

- **Bold**: `**text**`
- **Highlight**: `==text==` (Obsidian/extended Markdown)
- **Headings**: ATX style (`# H1` through `###### H6`)
- **Images**: Standard `![alt](url)` and Obsidian `![[file]]`
- **Code blocks**: Fenced with 3+ backticks or tildes
- **Display math**: `$$...$$` blocks
- **HTML tables**: `<table>...</table>`
- **Markdown tables**: Pipe-delimited with separator rows
- **Callout blocks**: `> [!info]`, `> [!success]`, etc.
- **Blockquotes**: `> text`
- **List items**: Unordered (`-`, `*`, `+`) and ordered (`1.`, `2.`)

## How it works

md-prune uses a two-pass line parser:

1. **Pass 1 (Block Classification)**: A state machine classifies each line
   into block types (heading, code block, paragraph, etc.). Multi-line
   constructs like code fences are tracked to avoid false positives.

2. **Pass 2 (Content Extraction)**: For text blocks (paragraphs, blockquotes,
   list items, callouts), regex patterns find bold and highlight spans.
   Structural blocks are kept or dropped based on CLI flags.

3. **Heading Pruning**: Headings with no extracted content beneath them
   are removed (unless `--keep-all-headings` is set).

## License

MIT
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README with installation, usage, and feature documentation"
```

---

## Task 25: Final pre-flight and push

**Step 1: Run the complete check suite**

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo build
cargo test
```

All four must pass with zero warnings or errors.

**Step 2: Verify the tool works end-to-end on all reference files**

```bash
cargo run -- references/example01.md | head -20
cargo run -- references/example02.md | head -20
cargo run -- references/example03.md | head -20
```

Verify output looks correct.

**Step 3: Check git status**

```bash
git status
git log --oneline
```

Verify all files are committed and the log shows a clean history.

**Step 4: Push**

```bash
git push -u origin claude/md-prune-cli-tool-S3GL4
```

If push fails due to network error, retry with exponential backoff:
```bash
sleep 2 && git push -u origin claude/md-prune-cli-tool-S3GL4
sleep 4 && git push -u origin claude/md-prune-cli-tool-S3GL4
sleep 8 && git push -u origin claude/md-prune-cli-tool-S3GL4
sleep 16 && git push -u origin claude/md-prune-cli-tool-S3GL4
```

---

## Summary: Complete task list

| Task | Part | Description | Key files |
|------|------|-------------|-----------|
| 1 | 1 | Initialize Rust project | `Cargo.toml`, `.gitignore` |
| 2 | 1 | Define shared types | `src/types.rs` |
| 3 | 1 | Define error types | `src/errors.rs` |
| 4 | 1 | CLI argument parsing | `src/cli.rs` |
| 5 | 2 | Parser — blank lines + paragraphs | `src/parser.rs` |
| 6 | 2 | Parser — headings | `src/parser.rs` |
| 7 | 2 | Parser — fenced code blocks | `src/parser.rs` |
| 8 | 2 | Parser — display math | `src/parser.rs` |
| 9 | 2 | Parser — HTML tables | `src/parser.rs` |
| 10 | 2 | Parser — callouts + blockquotes | `src/parser.rs` |
| 11 | 2 | Parser — images, lists, MD tables | `src/parser.rs` |
| 12 | 3 | Extractor — basic bold | `src/extractor.rs` |
| 13 | 3 | Extractor — highlight | `src/extractor.rs` |
| 14 | 3 | Extractor — nested markers | `src/extractor.rs` |
| 15 | 3 | Extractor — strip markers | `src/extractor.rs` |
| 16 | 3 | Extractor — escaped markers | `src/extractor.rs` |
| 17 | 3 | Extractor — process block lines | `src/extractor.rs` |
| 18 | 3 | Extractor — callout extraction | `src/extractor.rs` |
| 19 | 4 | Pruner — basic block filtering | `src/pruner.rs` |
| 20 | 4 | Pruner — heading pruning | `src/pruner.rs` |
| 21 | 4 | Wire main.rs | `src/main.rs` |
| 22 | 5 | Generate golden files | `tests/expected/` |
| 23 | 5 | Integration tests | `tests/integration.rs`, `src/lib.rs` |
| 24 | 5 | Write README | `README.md` |
| 25 | 5 | Final checks + push | — |

**Total:** 25 tasks across 5 plan parts, ~100+ unit tests, ~14 integration tests.
