---
icon: lucide/terminal
---

# CLI Reference

## Synopsis

```text
mlt [OPTIONS] <FILES>...
```

## Arguments

| Argument   | Description              |
| ---------- | ------------------------ |
| `<FILES>` | One or more MATLAB files to lint (required) |

## Options

| Option                | Description                                      |
| --------------------- | ------------------------------------------------ |
| `--fix`               | Apply automatic fixes where available            |
| `--config <PATH>`     | Path to config file (default: `.mlt.toml` in CWD) |
| `--preset <PRESET>`   | Preset to use (`all`, `mathworks`, `recommended`); overrides `[lint] preset` |
| `-h`, `--help`        | Print help information                           |
| `-V`, `--version`     | Print version information                        |

## Commands

### Lint files

Lint one or more MATLAB files and print diagnostics to stdout:

```bash
mlt src/compute.m src/utils.m
```

Output format:

```text
<file>:<line>:<column> [<severity>] <rule_id>: <message>
```

Example:

```text
src/compute.m:2:5 [W] M001: Statement without trailing semicolon may produce unintended console output
```

### Apply fixes

Run with `--fix` to automatically apply fixes:

```bash
mlt --fix src/compute.m
```

Output:

```text
Fixed 2 issues in src/compute.m
```

Only diagnostics with auto-fix support are applied. Issues without fixes are left unchanged and not reported in `--fix` mode.

### Explicit configuration

By default, mlt looks for `.mlt.toml` in the current working directory. Override with `--config`:

```bash
mlt --config configs/strict.toml src/**/*.m
```

If `--config` is specified and the file does not exist, mlt exits with an error. If no `--config` is given and `.mlt.toml` is not found, mlt uses the `mathworks` preset (see [Configuration](../configuration.md#presets)).

## Exit Codes

| Code | Meaning                              |
| ---- | ------------------------------------ |
| `0`  | No diagnostics found (or `--fix` succeeded) |
| `1`  | One or more diagnostics found        |
| `2`  | Runtime error (file not found, parse failure, bad config) |

## Shell Glob Expansion

mlt does not perform its own glob expansion. Use your shell's globbing:

```bash
# Bash/Zsh — lint all .m files recursively
mlt **/*.m

# Or use find
find src -name '*.m' -exec mlt {} +
```

## Examples

```bash
# Lint a single file
mlt main.m

# Lint all MATLAB files in a directory (using shell glob)
mlt src/**/*.m

# Fix issues with a custom config
mlt --fix --config .mlt-strict.toml lib/**/*.m

# Check version
mlt --version
```
