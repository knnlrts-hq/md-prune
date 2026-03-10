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
    let mut output_blocks: Vec<(BlockKind, Vec<String>)> = Vec::new();

    for block in &blocks {
        let output_lines = match &block.kind {
            BlockKind::BlankLine => Vec::new(),
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
            let has_content = blocks[i + 1..]
                .iter()
                .take_while(|(kind, _)| {
                    if let BlockKind::Heading { level: next_level } = kind {
                        // Lower-level headings (higher number) don't stop us
                        *next_level > level
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
///
/// A blank line is inserted between blocks UNLESS a heading is immediately
/// followed by non-heading content (headings flow directly into their section).
fn render_output(blocks: &[(BlockKind, Vec<String>)]) -> String {
    if blocks.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for (i, (kind, lines)) in blocks.iter().enumerate() {
        if i > 0 {
            // Skip blank line only when a heading is directly followed by
            // non-heading content (the heading's "body")
            let prev_is_heading = matches!(blocks[i - 1].0, BlockKind::Heading { .. });
            let curr_is_heading = matches!(kind, BlockKind::Heading { .. });
            let skip_blank = prev_is_heading && !curr_is_heading;
            if !skip_blank {
                output.push('\n');
            }
        }
        for line in lines {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Config;

    // -- Task 19: basic block filtering --

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

    // -- Task 20: heading pruning edge cases --

    #[test]
    fn prune_nested_headings_with_deep_content() {
        let input = "# Top\n## Empty Section\n## Section With Content\n**Bold here.**";
        let config = Config::default();
        let result = prune(input, &config);
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
}
