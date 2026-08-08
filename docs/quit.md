# QUIT - Analysis Did Not Complete

**Default severity:** Error
**Auto-fix:** No
**Category:** Incomplete Analysis
**Can be disabled:** No

## What this rule does

Emits an error when code analysis did not complete because the linter encountered an internal error while inspecting a file. This is an engine-level guard, not a registered lint rule: if a rule panics mid-analysis, mlt catches the failure, discards the partial diagnostics, and reports a single QUIT diagnostic instead of crashing.

## Why this matters

- **Signal integrity**: A file that was not fully analyzed may produce an incomplete or misleading diagnostic set, so the whole file is reported as failed rather than emitting partial results
- **Robustness**: An internal analyzer error is contained and reported, not propagated as a crash
- **Parity**: Mirrors MATLAB's Code Analyzer QUIT check ("Code analysis did not complete. Code Analyzer encountered an error."), which cannot be disabled

## Examples

### Incorrect (engine internal error)

```matlab
% Any file whose analysis aborts internally (e.g. a rule panicked during
% analysis) yields a single QUIT diagnostic at line 1, column 1.
```

Output:

```text
file.m:1:1 [E] QUIT: Code analysis did not complete. Code Analyzer encountered an error.
```

### Correct (no QUIT)

```matlab
x = 1;
```

A clean file lints normally with no QUIT diagnostic.

## Configuration

This check is not configurable and cannot be disabled. It is an engine guard implemented in `Linter::lint` (`crates/mlt_core/src/linter.rs`) via a panic guard (`std::panic::catch_unwind`), so it is not subject to the per-rule or per-category configuration system.

## Automatic fixes

None.

## Target node types

None. QUIT is not subscribed to any tree-sitter node type; it is produced by the lint engine itself.

## Related rules

- `EOFER` — Too many syntax errors
- `EOFMI` — Invalid syntax at end of file (incomplete file)
- `ENDCT` — Possible missing `end`
- `SYNER` — Parse error at a node

The related parse-level checks fire on user-code syntax problems; QUIT only fires on an internal analyzer failure and intentionally does not overlap with them.
