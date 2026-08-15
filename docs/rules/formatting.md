---
icon: lucide/align-left
---

# Formatting Suggestion Checks

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Formatting
**Can be disabled:** Yes

## What this rule does

Suggests formatting improvements for MATLAB code. All 7 checks share a
single `FormattingEngine` that runs as a file-level rule, walking the whole
tree once and applying every enabled check per node. Each diagnostic
carries the specific check ID (e.g. `NOCOMMA`, `NO4LP`). Individual checks
can be enabled, disabled, or reconfigured through their own
`[lint.rules.*]` tables.

## Check IDs

### NOCOMMA

Severity: **info** · Auto-fix: **yes**

Use commas to separate elements in a row

### NO4LP

Severity: **info** · Auto-fix: **yes**

Use 4-space indentation in loop/conditional bodies

### ALIGN

Severity: **info** · Auto-fix: **yes**

Align 'elseif'/'else' clauses with their 'if'

### NOPTS

Severity: **info** · Auto-fix: **yes**

Remove unnecessary parentheses around if/while conditions

### NOPRT

Severity: **info** · Auto-fix: **yes**

Remove unnecessary parentheses

### PRTCAL

Severity: **info** · Auto-fix: **yes**

Consider using command syntax instead of function syntax

### NCOMMA

Severity: **info** · Auto-fix: **yes**

Use a comma to separate input arguments

## Automatic fixes

Each check rewrites the flagged construct directly:

- NOCOMMA inserts a comma between space-separated row elements (`[1 2 3]` → `[1, 2, 3]`).
- NCOMMA inserts a comma between space-separated function arguments.
- NO4LP rewrites the indentation prefix of misindented statements.
- ALIGN re-indents `elseif`/`else` clauses to the column of their `if`.
- NOPTS and NOPRT strip unnecessary parentheses (`(x)` → `x`).
- PRTCAL rewrites a string-only call to command syntax (`disp('hello')` → `disp hello`).

## Examples

### Incorrect

```matlab
x = [1 2 3];
y = (a);
if (x > 0)
    disp('hello');
end
```

### Correct

```matlab
x = [1, 2, 3];
y = a;
if x > 0
    disp hello;
end
```

### Fixed

```diff
- x = [1 2 3];
+ x = [1, 2, 3];
- y = (a);
+ y = a;
- if (x > 0)
+ if x > 0
```

## Configuration

Each check is configured independently through its own rule table:

```toml
[lint.rules.NOCOMMA]
severity = "info"

[lint.rules.NO4LP]
severity = "info"
indent_size = 4

[lint.rules.PRTCAL]
severity = "off"
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
