---
icon: lucide/code
---

# NOSEMI - Trailing Semicolon Missing

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Formatting
**Can be disabled:** Yes

## What this rule does

Flags MATLAB statements that do not end with a trailing semicolon (`;`). This applies to statements that produce a value — primarily assignments, function calls, and commands at the statement level.

## Why this matters

- **Performance**: In loops, unsuppressed output causes significant slowdowns due to console I/O on every iteration
- **Noise**: Unintended console output clutters the Command Window, making it harder to find real output
- **Intent**: Without a semicolon, it's ambiguous whether the output was intentional (for debugging) or accidental
- **CI/CD**: Stray output in automated MATLAB scripts can pollute logs or interfere with output parsing

## Examples

### Correct

```matlab
x = compute_value();
data = load('file.mat');
result = process(data);

% Intentional output (configure ignore_functions)
disp('Processing complete');
fprintf('Result: %d\n', result);
```

### Incorrect

```matlab
x = compute_value()
data = load('file.mat')
result = process(data)
```

### Fixed

```matlab
x = compute_value();
data = load('file.mat');
result = process(data);
```

## Configuration

```toml title=".mlt.toml"
[lint.rules.NOSEMI]
severity = "info"
ignore_functions = ["disp", "fprintf", "warning", "error"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `ignore_functions` | array of strings | `[]` | Function names to exclude from this check |

### `ignore_functions`

A list of function names that are commonly used for intentional console output. When a `function_call` or `command` statement calls one of these functions, the rule will not flag it even without a semicolon.

Common values:

```toml
ignore_functions = [
    "disp",
    "fprintf",
    "warning",
    "error",
    "assert",
    "sprintf",  # usually assigned, but sometimes standalone
]
```

## Automatic fixes

This rule inserts a semicolon (`;`) immediately after the end of the statement:

```diff
- x = compute_value()
+ x = compute_value();
```

The fix is always safe — it suppresses output but does not change program behavior.

## Target node types

This rule triggers on the following tree-sitter node types when they appear at statement level (direct children of `source_file` or `block`):

- `assignment` — Variable assignments (`a = expr`)
- `function_call` — Function calls used as statements (`foo(x)`)
- `command` — Command-syntax calls (`cd dir`, `help topic`)

Sub-expressions (e.g., `bar(1)` inside `x = foo(bar(1))`) are **not** flagged.

## Related rules

- `NOCOMMA` — Missing comma between matrix elements
- `NO4LP` — Missing indentation for loop body
