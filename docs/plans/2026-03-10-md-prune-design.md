# md-prune Design Document

**Date:** 2026-03-10
**Status:** Approved

## Purpose

`md-prune` is a CLI tool that parses a Markdown file and outputs a pruned version
containing only bold (`**...**`) and/or highlighted (`==...==`) text, along with
optionally retained structural elements (headings, images, tables, code blocks,
display math, callout blocks).

**Primary use cases:**
1. **Study note distillation** — extract key sentences you've bolded/highlighted
   while reading, producing a condensed review document
2. **Document summarization** — reduce long markdown docs to their most important
   points for sharing or quick reference

## CLI Interface

```
md-prune [OPTIONS] <INPUT>

Arguments:
  <INPUT>                 Path to the input markdown file

Options:
  -o, --output <FILE>     Output file path (default: stdout)

  Filter mode (default: both bold and highlight):
    --bold-only           Only extract bold (**...**) text
    --highlight-only      Only extract highlighted (==...==) text

  Content to keep (all default to yes):
    --no-headings         Exclude headings
    --no-images           Exclude images
    --no-tables           Exclude tables (markdown and HTML)
    --no-code-blocks      Exclude fenced code blocks
    --no-math             Exclude display math blocks ($$...$$)
    --no-callouts         Exclude callout blocks (> [!type])

  Extraction behavior:
    --keep-paragraph      Keep entire paragraph if any part is bold/highlighted
    --keep-all-headings   Keep all headings, even for empty sections
    --strip-markers       Remove bold/highlight markers from output text

  -h, --help              Print help
  -V, --version           Print version
```

**Design rationale:**
- `--no-*` pattern follows Unix convention (features on by default, flags to disable)
- Single file input only — composable with shell tools (`xargs`, `find`)
- stdout default enables piping (`md-prune input.md | less`)

## Architecture: Two-Pass Line Parser

### Pass 1: Block Classification

A line-by-line scanner classifies the document into blocks using a state machine.

| Block Type | Detection | Behavior |
|---|---|---|
| Heading | Lines starting with `#` (1-6) | Kept/dropped based on `--no-headings` + content pruning |
| Fenced Code Block | Lines starting with 3+ backticks or tildes | Entire block kept/dropped via `--no-code-blocks` |
| Display Math | Lines starting with `$$` | Entire block kept/dropped via `--no-math` |
| Callout Block | Lines starting with `> [!` | Kept/dropped via `--no-callouts`; extraction applies within |
| Blockquote | Lines starting with `>` (not callouts) | Treated like paragraphs for extraction |
| HTML Table | `<table` through `</table>` | Entire block kept/dropped via `--no-tables` |
| Markdown Table | `|` delimiters + separator row | Entire block kept/dropped via `--no-tables` |
| Image | `![[...]]` or `![...](...)` | Kept/dropped via `--no-images` |
| List Item | Lines starting with `-`, `*`, `+`, or `N.` | Extract within, preserve list formatting |
| Paragraph | Everything else | Core extraction target |

**State machine details:**
- Tracks whether we're inside a code fence, display math block, HTML table, or callout
- Code fences track the exact backtick/tilde count (3, 4, 5, etc.) — closing fence must
  match or exceed the opening count
- While inside a code fence, no other detection runs (prevents false positives)

### Pass 2: Content Extraction

For paragraphs, blockquotes, and list items:

**Bold span detection:** `**...**` where content can contain any inline elements
(inline code, math, links, highlight markers) but cannot span across blank lines.

**Highlight span detection:** `==...==` with the same rules.

**Extraction modes:**
1. **Default:** Extract only marked spans. `Hello **world** bar` → `**world**`
2. **`--keep-paragraph`:** If any bold/highlight exists, keep entire paragraph verbatim
3. **`--strip-markers`:** Post-processing removes `**` and `==` delimiters from output

**Multi-line bold spans:** Consecutive non-blank lines within a paragraph are joined
before extraction, then output preserves original line breaks.

**Nested markers:** `**==text==**` — both markers match. In `--bold-only` mode,
the outer `**...**` captures everything. In `--highlight-only` mode, inner `==text==`
is extracted.

**Heading pruning:** After pass 2, headings with no extracted content beneath them
(before the next heading of equal or higher level) are dropped — unless
`--keep-all-headings` is set.

## Module Structure

```
md-prune/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point, CLI arg parsing, I/O
│   ├── cli.rs           # clap derive structs for CLI args
│   ├── parser.rs        # Pass 1: block classification + state machine
│   ├── extractor.rs     # Pass 2: bold/highlight span extraction
│   ├── pruner.rs        # Orchestrates parsing + extraction + heading pruning
│   └── types.rs         # Shared types (Block enum, Config struct, etc.)
├── tests/
│   ├── integration.rs   # Integration tests using reference files
│   └── expected/        # Golden file outputs for snapshot testing
├── references/          # Example input files (existing)
└── README.md
```

## Dependencies

| Crate | Purpose |
|---|---|
| `clap` (derive) | CLI argument parsing |
| `regex` | Bold/highlight span matching |
| `thiserror` | Custom error type definitions |
| `anyhow` | Application-level error handling with context |

Four crates total. No progress bars needed (single-file processing completes
in milliseconds).

## Error Handling

**Custom error types via `thiserror`:**
- `FileNotFound` — input file doesn't exist
- `ReadError` — I/O error reading file
- `WriteError` — I/O error writing output
- `ParseError` — malformed markdown (unclosed code fence at EOF, etc.)

**Edge cases:**
1. Empty file → empty output (no error)
2. No bold/highlight text found → empty output with warning to stderr
3. Unclosed code fence at EOF → treat remaining lines as inside code block
4. Unclosed bold/highlight at line end → treat as literal `**` or `==`
5. Escaped markers (`\*\*`, `\=\=`) → literal text, not markers
6. UTF-8 with BOM → handle transparently
7. Mixed line endings (CRLF/LF) → normalize during parsing
8. Code fences with 3+ backticks/tildes → track exact count for matching

**Exit codes:** 0 = success, 1 = I/O error, 2 = invalid CLI args (clap)

## Testing Strategy

**Unit tests (per module):**
- `parser.rs`: Block classification for every type, code fences with 3/4/5+ backticks,
  state machine transitions
- `extractor.rs`: Bold extraction, highlight extraction, nested markers,
  `--strip-markers`, `--keep-paragraph`, escaped markers, unclosed markers
- `pruner.rs`: Heading pruning (empty sections removed, `--keep-all-headings`)

**Integration tests (reference files):**
- Default flags on all 3 reference files
- Each `--no-*` flag individually
- `--bold-only` and `--highlight-only` on example01.md (has both markers)
- `--output` flag writes to file
- stdout piping

**Golden file / snapshot testing:** Expected outputs stored in `tests/expected/`
for regression detection.
