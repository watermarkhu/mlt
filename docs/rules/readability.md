---
icon: lucide/eye
---

# Readability Improvements

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

A multi-check rule engine that detects readability improvements in MATLAB
code. Instead of implementing one struct per check, a single
`ReadabilityEngine` dispatches to per-check-ID logic inside its `check()`
method (and a file-level pass for MFAMB), emitting diagnostics with the
specific check ID (e.g. `ISCHR`, `IJCL`, `RPMT1`).

The 35 checks cover type-checking simplification (`isa(x, 'type')` →
dedicated predicates), dimension checks (`size(x,dim)==1` → `isrow`/`iscolumn`),
variable shadowing, unnecessary brackets, modern string functions,
character literals, formatting, control-flow style, error/warning message
IDs, input validation, output style, flip/rotate, redundant arithmetic and
boolean logic, size and logical helpers, `arguments` attributes, and
ambiguous identifiers.

## Check IDs

### ASGSL

Severity: **info** · Auto-fix: **no**

Assignment inside a conditional expression

### COMNL

Severity: **info** · Auto-fix: **yes**

Newline following comma acts as a row separator in a matrix

### SPERR

Severity: **info** · Auto-fix: **no**

Prefer a message identifier for `error`

### SPWRN

Severity: **info** · Auto-fix: **no**

Prefer a message identifier for `warning`

### NCHKE

Severity: **info** · Auto-fix: **no**

Use `narginchk`/`nargoutchk` for argument validation

### DSPSP

Severity: **info** · Auto-fix: **yes**

Prefer `fprintf` over `disp(sprintf(...))`

### DSPSY

Severity: **info** · Auto-fix: **no**

Prefer `disp` over `display`

### STLOW

Severity: **info** · Auto-fix: **yes**

Unnecessary UPPER/LOWER call in a comparison

### FLUDLR

Severity: **info** · Auto-fix: **yes**

Use `rot90(x, 2)` instead of `flipud(fliplr(x))`/`fliplr(flipud(x))`

### RPMT1

Severity: **info** · Auto-fix: **yes**

Trivial multiplication by 1

### RPMT0

Severity: **info** · Auto-fix: **yes**

Multiplication by 0

### RPMTT

Severity: **info** · Auto-fix: **yes**

Boolean tautology (`x \|\| true`)

### RPMTF

Severity: **info** · Auto-fix: **yes**

Boolean contradiction (`x && false`)

### RPMTI

Severity: **info** · Auto-fix: **yes**

Trivial addition of 0

### RPMTN

Severity: **info** · Auto-fix: **yes**

Trivial subtraction of 0

### PSIZE

Severity: **info** · Auto-fix: **yes**

Use `numel(x)` instead of `prod(size(x))`

### LOGSUM

Severity: **info** · Auto-fix: **no**

Use `any` instead of `sum(logical) > 0`

### LOGL

Severity: **info** · Auto-fix: **no**

Use logical indexing instead of `x(find(condition))`

### ISCHR

Severity: **info** · Auto-fix: **yes**

Use `ischar(x)` instead of `isa(x, 'char')`

### ISSTR

Severity: **info** · Auto-fix: **yes**

Use `isstring(x)` instead of `isa(x, 'string')`

### ISLOG

Severity: **info** · Auto-fix: **yes**

Use `islogical(x)` instead of `isa(x, 'logical')`

### ISCEL

Severity: **info** · Auto-fix: **yes**

Use `iscell(x)` instead of `isa(x, 'cell')`

### IJCL

Severity: **info** · Auto-fix: **no**

`i`/`j` used as a variable (shadows the complex unit)

### ISMAT

Severity: **info** · Auto-fix: **yes**

Use `isnumeric(x)` instead of `isa(x, 'double')`

### ISROW

Severity: **info** · Auto-fix: **yes**

Use `isrow(x)` instead of `size(x, 1) == 1`

### ISCOL

Severity: **info** · Auto-fix: **yes**

Use `iscolumn(x)` instead of `size(x, 2) == 1`

### NBRAK2

Severity: **info** · Auto-fix: **yes**

Unnecessary brackets around a scalar expression

### MFAMB

Severity: **info** · Auto-fix: **no**

Cannot determine whether a name is a variable or function

### FVINR

Severity: **info** · Auto-fix: **yes**

Add an `(Input)` attribute to `arguments` blocks for readability

### STREMP

Severity: **info** · Auto-fix: **yes**

Use `strlength(s)==0` instead of `strcmp(s, '')`

### STRCL1

Severity: **info** · Auto-fix: **no**

Use `startsWith`/`endsWith` instead of `strncmp`/`strncmpi`

### STRCLFH

Severity: **info** · Auto-fix: **no**

Use `contains` instead of `strfind` for presence checks

### STRIFCND

Severity: **info** · Auto-fix: **no**

Simplify if-conditions involving string comparisons

### CHARTEN

Severity: **info** · Auto-fix: **yes**

Use `newline` instead of `char(10)`

### SPRINTFN

Severity: **info** · Auto-fix: **yes**

Use `num2str` over simple `sprintf` for number formatting

## Automatic fixes

Rewrites the flagged construct into the clearer equivalent:

- `isa(x, 'type')` → `ischar`/`isstring`/`islogical`/`iscell`/`isnumeric`
  (ISCHR, ISSTR, ISLOG, ISCEL, ISMAT).
- `size(x, dim) == 1` → `isrow(x)`/`iscolumn(x)` (ISROW, ISCOL).
- `x * 1` → `x`, `x * 0` → `zeros(size(x))`, `x + 0` → `x`,
  `x - 0` → `x` (RPMT1, RPMT0, RPMTI, RPMTN).
- `x | true` → `true` and `x & false` → `false` (RPMTT, RPMTF).
- `prod(size(x))` → `numel(x)`, `char(10)` → `newline`,
  `sprintf('%d', x)` → `num2str(x)`, `strcmp(s, '')` → `strlength(s)==0`.
- `flipud(fliplr(x))` → `rot90(x, 2)`, `[scalar]` → `scalar`,
  `disp(sprintf(...))` → `fprintf(...)`.
- A trailing comma before a newline in a matrix becomes a semicolon (COMNL).
- An `(Input)` attribute is inserted into attribute-less `arguments`
  blocks (FVINR).

## Examples

### Incorrect

```matlab
if isa(x, 'char')
if a || true
y = flipud(fliplr(m));
n = prod(size(a));
```

### Correct

```matlab
if ischar(x)
if a
y = rot90(m, 2);
n = numel(a);
```

### Fixed

```diff
- n = prod(size(a));
+ n = numel(a);
```

## Configuration

```toml
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["IJCL", "NBRAK2"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
