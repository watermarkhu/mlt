---
icon: lucide/trash-2
---

# Unused Constructions

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unused Constructions
**Can be disabled:** Yes

## What this rule does

Detects unused variables, functions, methods, and dead code in MATLAB
files. A single file-level engine (`UnusedEngine`) builds a symbol table
and runs unreachability analysis, then cross-references every definition
against its usages to identify dead code. It covers 17 checks from
MATLAB's "Unused Constructions" category; each diagnostic carries the
specific check ID (e.g. `NASGU`, `UNRCH`, `MSNU`).

## Check IDs

### NASGU

Severity: **warning** · Auto-fix: **no**

Variable is assigned but never used

### NUSED

Severity: **warning** · Auto-fix: **no**

Variable is defined (input arg) but never used

### NOEFF

Severity: **warning** · Auto-fix: **no**

Statement has no effect (expression result discarded)

### EQEFF

Severity: **warning** · Auto-fix: **no**

Comparison has no effect (result not used)

### ASGLU

Severity: **warning** · Auto-fix: **no**

Assignment to a variable that is immediately overwritten

### SETNU

Severity: **warning** · Auto-fix: **no**

Output of function assigned but never used

### PUSE

Severity: **warning** · Auto-fix: **no**

Persistent/global variable set but not used

### PREALL

Severity: **warning** · Auto-fix: **no**

Variable preallocated but unused

### INUSA

Severity: **warning** · Auto-fix: **no**

Input argument not used in function

### INUSD

Severity: **warning** · Auto-fix: **no**

Input argument defined but could be removed

### VANUS

Severity: **info** · Auto-fix: **no**

Value assigned to `ans` is unused

### DEFNU

Severity: **warning** · Auto-fix: **no**

Local function defined but never called

### UNRCH

Severity: **warning** · Auto-fix: **no**

Unreachable code after return/break/continue

### MANU

Severity: **warning** · Auto-fix: **no**

Method defined but never called

### VUNUS

Severity: **warning** · Auto-fix: **no**

Variable assigned in all branches but unused after

### MSNU

Severity: **warning** · Auto-fix: **no**

Struct field set but never read

### MSNE

Severity: **info** · Auto-fix: **no**

Struct field doesn't exist (assigned but possibly a typo)

## Examples

### Incorrect

```matlab
function f()
    temp = compute();   % NASGU: assigned but never used
    x = 1;
    return;
    y = x;              % UNRCH: unreachable after return
    s.field = 1;        % MSNU: set but never read
end
```

### Correct

```matlab
function f()
    x = compute();
    disp(x);
    s.field = x;
    disp(s.field);
end
```

## Configuration

```toml
[lint.rules.UNUSED_ENGINE]
severity = "warning"
ignore_patterns = ["_*", "unused*"]
disabled_checks = ["VANUS"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
