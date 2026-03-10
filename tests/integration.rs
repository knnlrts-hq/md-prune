use std::fs;

use md_prune::pruner::prune;
use md_prune::types::{Config, FilterMode};

/// Read a file, prune with the given config, and compare to expected output.
///
/// On mismatch, prints a line-level diff showing the first divergent line.
fn assert_prune_matches(input_path: &str, expected_path: &str, config: &Config) {
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to read input {input_path}: {e}"));
    let expected = fs::read_to_string(expected_path)
        .unwrap_or_else(|e| panic!("Failed to read expected {expected_path}: {e}"));

    let actual = prune(&input, config);

    if actual != expected {
        let actual_lines: Vec<&str> = actual.lines().collect();
        let expected_lines: Vec<&str> = expected.lines().collect();

        let max_lines = actual_lines.len().max(expected_lines.len());
        for i in 0..max_lines {
            let a = actual_lines.get(i).unwrap_or(&"<missing>");
            let e = expected_lines.get(i).unwrap_or(&"<missing>");
            if a != e {
                panic!(
                    "Mismatch at line {i} (0-indexed) for {input_path}:\n  \
                     expected: {e:?}\n  actual:   {a:?}\n\nFull actual output:\n{actual}"
                );
            }
        }

        panic!(
            "Output line count mismatch for {input_path}: expected {} lines, got {}",
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
