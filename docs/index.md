---
icon: lucide/flame
---

# mlt

## An ultra-fast, extensible linter for MATLAB, written in Rust

<div class="grid cards" markdown>

-   :zap:{ .lg .middle } **Built for speed**

    ---

    Single-pass AST traversal using tree-sitter. Lint thousands of files in milliseconds.

    [:octicons-arrow-right-24: Quick Start](getting-started/quickstart.md)

-   :puzzle_piece:{ .lg .middle } **Extensible rule system**

    ---

    Clean `Rule` trait architecture inspired by rumdl. Add new rules with minimal boilerplate.

    [:octicons-arrow-right-24: Rules Reference](../rules.md)

-   :wrench:{ .lg .middle } **Auto-fix support**

    ---

    Rules can provide automatic fixes. Run `mlt --fix` to apply them in one shot.

    [:octicons-arrow-right-24: CLI Reference](usage/cli.md)

-   :gear:{ .lg .middle } **Configurable**

    ---

    TOML-based configuration. Enable, disable, or override severity per rule. Per-rule parameters.

    [:octicons-arrow-right-24: Configuration](configuration.md)

</div>

## Quick Start

```bash
# Install from source
cargo install --path crates/mlt_cli

# Lint MATLAB files
mlt path/to/file.m

# Auto-fix issues
mlt --fix path/to/file.m
```

## Example Output

```text
src/compute.m:2:5 [W] M001: Statement without trailing semicolon may produce unintended console output
src/compute.m:4:5 [W] M001: Statement without trailing semicolon may produce unintended console output

Found 2 issues in 1 file.
```

## Design Principles

- **Single-pass traversal** — The AST is walked exactly once. Rules subscribe to node types and are dispatched via an O(1) registry lookup.
- **tree-sitter powered** — Leverages `tree-sitter-matlab` for fast, incremental, error-tolerant parsing.
- **Workspace architecture** — Separated into `mlt_core` (engine), `mlt_rules` (rule implementations), and `mlt_cli` (interface).
- **Inspired by the best** — Architecture modeled after [ruff](https://github.com/astral-sh/ruff) and [rumdl](https://github.com/rvben/rumdl).

## Next Steps

<div class="grid cards" markdown>

-   [:octicons-download-24: **Installation**](getting-started/installation.md)

    Build and install mlt from source.

-   [:octicons-play-24: **Quick Start**](getting-started/quickstart.md)

    Get up and running in minutes.

-   [:octicons-book-24: **Rules Reference**](../rules.md)

    Explore all available linting rules.

-   [:octicons-gear-24: **Configuration**](configuration.md)

    Customize mlt for your project with `.mlt.toml`.

</div>
