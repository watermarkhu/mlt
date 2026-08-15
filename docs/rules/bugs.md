---
icon: lucide/bug
---

# Bugs

**Default severity:** Error
**Auto-fix:** Yes
**Category:** Bugs
**Can be disabled:** Yes

## What this rule does

Detects likely bugs and logic errors in MATLAB code. All 35 checks share a
single hybrid engine (`BugsEngine`) that performs both node-level checking
(dispatched per-node during traversal) and file-level checking (full-tree
analysis after traversal). Each diagnostic carries the specific check ID
(e.g. `IFBDUP`, `FNAN`).

## Check IDs

### IFBDUP

Severity: **error** · Auto-fix: **no**

Duplicate if-branch bodies

### IFCDUP

Severity: **error** · Auto-fix: **no**

Duplicate if-branch conditions

### CTRUE

Severity: **error** · Auto-fix: **no**

Condition is always true (`if true`, `while 1`)

### CFALSE

Severity: **error** · Auto-fix: **no**

Condition is always false (`if false`, `while 0`)

### SHOCIRT

Severity: **error** · Auto-fix: **no**

Short-circuit `&&` with non-scalar LHS

### SHOCIRF

Severity: **error** · Auto-fix: **no**

Short-circuit `\|\|` with non-scalar LHS

### DEBUGFUN

Severity: **error** · Auto-fix: **no**

Debug function in code (keyboard, dbstop, etc.)

### INCR

Severity: **error** · Auto-fix: **no**

Suspicious self-increment `x = x + 1`

### DECR

Severity: **error** · Auto-fix: **no**

Suspicious self-decrement `x = x - 1`

### CMDAND

Severity: **error** · Auto-fix: **yes**

`&` used where `&&` intended (boolean context)

### CMDOR

Severity: **error** · Auto-fix: **yes**

`\|` used where `\|\|` intended (boolean context)

### RHSFN

Severity: **error** · Auto-fix: **yes**

Function name used on RHS without `@`

### FNAN

Severity: **error** · Auto-fix: **yes**

Comparison with NaN (use `isnan` instead)

### LOGEMP

Severity: **error** · Auto-fix: **yes**

`length(x) == 0` instead of `isempty(x)`

### STCUL

Severity: **error** · Auto-fix: **no**

`strcmpi` with same-case arguments

### LBODUP

Severity: **error** · Auto-fix: **no**

Duplicate case values in switch

### FUNFUN

Severity: **error** · Auto-fix: **no**

Passing function name as string instead of handle

### DEFSIZE

Severity: **error** · Auto-fix: **yes**

`size(x) == [m n]` instead of `isequal(size(x), [m n])`

### VARARG

Severity: **error** · Auto-fix: **no**

Misuse of varargin/varargout

### STRCMPCSTR

Severity: **error** · Auto-fix: **no**

`strcmp` with single-char comparison

### ASSRT

Severity: **error** · Auto-fix: **no**

`assert` with constant true condition

### BDSCA2

Severity: **error** · Auto-fix: **no**

Suspicious scalar/array operation

### NOPRC

Severity: **error** · Auto-fix: **no**

No `otherwise` in switch

### MOCUP

Severity: **error** · Auto-fix: **no**

Operator precedence issue

### MDUPC

Severity: **error** · Auto-fix: **no**

Duplicate case in switch

### MNANC

Severity: **error** · Auto-fix: **yes**

Comparison with NaN (alternate form)

### MULCC

Severity: **error** · Auto-fix: **yes**

Multiple conditions could be simplified

### MEXCEP

Severity: **error** · Auto-fix: **no**

Catch without identifier

### PFUIXE

Severity: **error** · Auto-fix: **no**

Parfor index used in eval

### PFBFN

Severity: **error** · Auto-fix: **no**

Builtin function in parfor

### PFWHOS

Severity: **error** · Auto-fix: **no**

who/whos in parfor

### PFTUSE

Severity: **error** · Auto-fix: **no**

Temporary variable misuse in parfor

### PFRNC

Severity: **error** · Auto-fix: **no**

Reduction not consistent in parfor

### FWPARF

Severity: **error** · Auto-fix: **no**

For loop could be parfor

### PFTRIV

Severity: **error** · Auto-fix: **no**

Parfor could be for

## Automatic fixes

Rewrites the flagged construct into the safe equivalent. For example:

- `&` → `&&` and `|` → `||` in boolean contexts.
- `x == NaN` → `isnan(x)` (and `x ~= NaN` → `~isnan(x)`).
- `length(x) == 0` → `isempty(x)` and `size(x) == [m n]` → `isequal(size(x), [m n])`.
- Bare function names on the RHS of an assignment get an `@` prefix.
- Duplicate boolean conditions are collapsed (`a && a` → `a`).

## Examples

### Incorrect

```matlab
if x == NaN
    disp('not a number');
end
while true
    % infinite
end
```

### Correct

```matlab
if isnan(x)
    disp('not a number');
end
while true % intentional
    % ...
end
```

## Configuration

```toml
[lint.rules.BUGS_ENGINE]
severity = "error"
debug_functions = ["keyboard", "dbstop", "dbclear", "dbcont", "dbquit", "dbup", "dbdown"]
higher_order_functions = ["cellfun", "arrayfun", "structfun", "bsxfun", "spfun"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
