---
icon: lucide/zap
---

# Performance Improvement Checks

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Performance
**Can be disabled:** Yes

## What this rule does

A hybrid rule module implementing 41 performance-related MATLAB lint
checks from MATLAB's Code Analyzer. Most checks operate at the node level
(function calls, operators, commands), while AGROW, SAGROW, and PFBNS
require file-level traversal to detect array-growth patterns inside loops.
Each diagnostic carries the specific check ID (e.g. `AGROW`, `MINV`).

A single `PerformanceEngine` struct handles all 41 checks. Node-level
checks are dispatched by node kind (`function_call`, `command`,
`binary_operator`, `boolean_operator`, `comparison_operator`,
`unary_operator`); file-level checks traverse the full tree looking for
growth patterns inside `for_statement`/`while_statement`. The node-level
`check_*` methods and their tests live in sibling `check_*.rs` modules;
this module keeps the engine, the dispatch, the file-level growth walk
(AGROW/SAGROW/PFBNS), and the shared free helpers.

## Check IDs

### AGROW

Severity: **info** · Auto-fix: **no**

Variable appears to grow inside a loop; consider preallocating

### SAGROW

Severity: **info** · Auto-fix: **no**

Struct field appears to grow inside a loop; consider preallocating

### AND2

Severity: **info** · Auto-fix: **yes**

Use `&&` (short-circuit) instead of `&` for scalar logical operations

### OR2

Severity: **info** · Auto-fix: **yes**

Use `\|\|` (short-circuit) instead of `\|` for scalar logical operations

### MINV

Severity: **info** · Auto-fix: **no**

Use `A\b` instead of `inv(A)*b` for better numerical stability and performance

### GFLD

Severity: **info** · Auto-fix: **no**

Use dynamic field names `s.(name)` instead of `getfield`

### SFLD

Severity: **info** · Auto-fix: **no**

Use dynamic field names `s.(name) = val` instead of `setfield`

### EXIST

Severity: **info** · Auto-fix: **no**

Use `isfile` or `isfolder` instead of `exist(..., 'file')`

### PFBNS

Severity: **info** · Auto-fix: **no**

Array initialized as empty then grown; preallocate for known size

### CCAT

Severity: **info** · Auto-fix: **no**

Use string concatenation or `join` instead of repeated `strcat`

### CCAT1

Severity: **info** · Auto-fix: **no**

Consider using `join` for cell array of character vector concatenation

### ISMT

Severity: **info** · Auto-fix: **no**

Use `isempty(x)` instead of `length(x)==0`

### ISCL

Severity: **info** · Auto-fix: **no**

Use `isscalar(x)` instead of `length(x)==1`

### ST2NM

Severity: **info** · Auto-fix: **yes**

Use `str2double` instead of `str2num` for performance and security

### FLPST

Severity: **info** · Auto-fix: **yes**

Use `flip` instead of `flipud`/`fliplr` on vectors

### MXFND

Severity: **info** · Auto-fix: **no**

Use `max(x,[],'all')` instead of nested `max(max(x))`

### EFIND

Severity: **info** · Auto-fix: **no**

Use logical indexing instead of `find` when used as a subscript

### UDIM

Severity: **info** · Auto-fix: **no**

Specify the dimension argument in `sum`/`max`/`min`/`prod`/`mean`

### FREAD

Severity: **info** · Auto-fix: **no**

Specify the precision argument in `fread` for performance

### N2UNI

Severity: **info** · Auto-fix: **no**

Consider using `unique` instead of `setdiff`+`union` patterns

### TNMLP

Severity: **info** · Auto-fix: **no**

Move `tic`/`toc` outside the loop body for accurate timing

### LAXES

Severity: **info** · Auto-fix: **no**

Cache the axes handle returned by `gca`/`gcf` instead of repeated calls

### MMTC

Severity: **info** · Auto-fix: **no**

Use `.^2` instead of `.*` with the same operand

### MRPBW

Severity: **info** · Auto-fix: **yes**

Use `imbinarize` instead of deprecated `im2bw`

### SPRIX

Severity: **info** · Auto-fix: **no**

Avoid indexing sparse matrices with full logical arrays

### TRSRT

Severity: **info** · Auto-fix: **no**

Use `mink`/`maxk` instead of sorting then indexing

### GRIDD

Severity: **info** · Auto-fix: **no**

Consider using `meshgrid` or `ndgrid` for grid generation

### CLALL

Severity: **info** · Auto-fix: **no**

`clear all` also clears breakpoints; use `clearvars` instead

### CLCLS

Severity: **info** · Auto-fix: **no**

`clear classes` is a slow operation; avoid in production code

### CLFUNC

Severity: **info** · Auto-fix: **no**

`clear functions` is a slow operation; avoid in production code

### CLJAVA

Severity: **info** · Auto-fix: **no**

`clear java` is a slow operation; avoid in production code

### CLMEX

Severity: **info** · Auto-fix: **no**

`clear mex` clears all MEX files from memory; use specific names

### CLEAR0ARGS

Severity: **info** · Auto-fix: **no**

`clear` with no arguments clears all variables; use `clearvars` instead

### RGXP1

Severity: **info** · Auto-fix: **no**

Regex pattern can be simplified for performance

### RGXPI

Severity: **info** · Auto-fix: **no**

Use `regexpi` instead of `regexp` with the `ignorecase` option

### TRIM1

Severity: **info** · Auto-fix: **yes**

Use `strtrim` instead of `deblank` for more thorough whitespace removal

### TRIM2

Severity: **info** · Auto-fix: **yes**

Use `strip` instead of `strtrim` for more flexible whitespace removal

### STTOK

Severity: **info** · Auto-fix: **no**

Use `split` instead of `strtok` in a loop for better performance

### STNCI

Severity: **info** · Auto-fix: **no**

Use `strcmpi` instead of wrapping `strcmp` with `lower`

### STCCS

Severity: **info** · Auto-fix: **no**

Use `contains` instead of `~isempty(strfind(...))`

### FNDSB

Severity: **info** · Auto-fix: **yes**

Use `contains` or `matches` instead of `findstr`

## Automatic fixes

Rewrites the flagged call or operator into its faster equivalent:

- `&` → `&&` and `|` → `||` in boolean contexts (AND2, OR2).
- `str2num` → `str2double` (ST2NM), `flipud`/`fliplr` → `flip` (FLPST).
- `im2bw` → `imbinarize` (MRPBW), `deblank` → `strtrim` (TRIM1),
  `strtrim` → `strip` (TRIM2), `findstr` → `contains` (FNDSB).

## Examples

### Incorrect

```matlab
for i = 1:n
    x = [x, val];          % AGROW
end
if a & b                   % AND2
inv(A) * b                 % MINV
exist('foo', 'file')       % EXIST
```

### Correct

```matlab
x = zeros(1, n);
for i = 1:n
    x(i) = val;
end
if a && b
A \ b
isfile('foo')
```

### Fixed

```diff
- if a & b
+ if a && b
```

## Configuration

```toml
[lint.rules.PERFORMANCE_ENGINE]
severity = "info"
skip_checks = ["AGROW", "ST2NM"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
