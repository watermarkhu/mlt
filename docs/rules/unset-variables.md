---
icon: lucide/alert-triangle
---

# Unset Variables

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unset Variables
**Can be disabled:** Yes

## What this rule does

Detects variables that might not be defined before use. A single file-level
engine (`UnsetVariablesEngine`) builds a symbol table and runs
definite-assignment analysis to find variables used on code paths where
they may never have been assigned. It covers 6 checks from MATLAB's "Unset
Variables" category; each diagnostic carries the specific check ID (e.g.
`NODEF`, `PSET`, `STOUT`).

## Check IDs

### NODEF

Severity: **warning** · Auto-fix: **no**

Variable might not be defined before use

### USENS

Severity: **warning** · Auto-fix: **no**

Variable used but might not be set in all code paths

### PSET

Severity: **warning** · Auto-fix: **no**

Variable set in one branch but not others

### SUSENS

Severity: **warning** · Auto-fix: **no**

Script variable used before set

### SVNODEF

Severity: **warning** · Auto-fix: **no**

Variable in script might not be defined

### STOUT

Severity: **warning** · Auto-fix: **no**

Output variable might not be assigned

## Examples

### Incorrect

```matlab
function f()
    disp(x);    % NODEF: x may never be defined before use
end

function y = g()
    if flag
        y = 1;
    end         % STOUT: output not assigned on all paths
end
```

### Correct

```matlab
function f()
    x = 0;
    disp(x);
end

function y = g()
    y = 0;
    if flag
        y = 1;
    end
end
```

## Configuration

```toml
[lint.rules.UNSET_VARIABLES_ENGINE]
severity = "warning"
ignore = ["myGlobalWorkspaceVar"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
