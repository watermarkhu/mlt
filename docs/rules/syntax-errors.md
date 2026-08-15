---
icon: lucide/x-circle
---

# Syntax Errors

**Default severity:** Error
**Auto-fix:** Yes
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Detects syntax errors in MATLAB source files. The engine combines
tree-sitter `ERROR`/`MISSING` node analysis with semantic checks the parser
cannot catch on its own: invalid operators, malformed file and function
names, structural violations, unterminated strings and comments, and
invalid numeric literals.

A single file-level engine (`SyntaxErrorsEngine`) dispatches all 48 checks
in one pass over the parse tree, plus targeted source scans that skip over
comments and strings. Each diagnostic carries the specific check ID
(e.g. `SYNER`, `BADNE`, `SEPEXR`).

## Check IDs

### SYNER

Severity: **error** · Auto-fix: **no**

ERROR node detected in parse tree

### BDFIL

Severity: **error** · Auto-fix: **no**

File name doesn't follow MATLAB naming rules

### BADNE

Severity: **error** · Auto-fix: **yes**

Source contains `!=` (MATLAB uses `~=`)

### BADOT

Severity: **error** · Auto-fix: **no**

Source contains `..` not part of `...`

### TWOCM

Severity: **error** · Auto-fix: **no**

Source contains `,,`

### CLIS

Severity: **error** · Auto-fix: **no**

`classdef` in a script file

### CLTWO

Severity: **error** · Auto-fix: **no**

Multiple `classdef` blocks in one file

### SOFOC

Severity: **error** · Auto-fix: **no**

Statements outside a class definition in a class file

### SEMFU

Severity: **error** · Auto-fix: **no**

File has only empty statements (only `;` and whitespace)

### FNDOT

Severity: **error** · Auto-fix: **no**

Function name contains dots outside a class methods block

### FNSWA

Severity: **error** · Auto-fix: **no**

Function name doesn't start with an alphabetic character

### NOPAR2

Severity: **error** · Auto-fix: **no**

Missing closing bracket mid-file

### EOLPAR

Severity: **error** · Auto-fix: **no**

Missing closing bracket at end of line

### ENDPAR

Severity: **error** · Auto-fix: **no**

Missing closing bracket at end of file

### ENDCT

Severity: **error** · Auto-fix: **no**

ERROR node suggesting a missing `end`

### ENDCT2

Severity: **error** · Auto-fix: **no**

An `end` might be missing after a block-opening keyword

### ENDCT3

Severity: **error** · Auto-fix: **no**

An `end` might be missing before a block-opening keyword

### ENDCT4

Severity: **error** · Auto-fix: **no**

A METHODS block or `end` might be missing before a function definition

### EOFMI

Severity: **error** · Auto-fix: **no**

File ends with an ERROR node (incomplete)

### NOLHS

Severity: **error** · Auto-fix: **no**

Assignment with empty left side

### BADCH

Severity: **error** · Auto-fix: **no**

Invalid control characters in source

### BADSP

Severity: **error** · Auto-fix: **no**

Non-ASCII whitespace characters in source

### BADCT

Severity: **error** · Auto-fix: **no**

Unicode explicit directional formatting characters

### REDEF

Severity: **error** · Auto-fix: **no**

Same identifier used as both function name and variable

### SEPEXR

Severity: **error** · Auto-fix: **yes**

Missing newline/semicolon between statements

### SBTMP

Severity: **error** · Auto-fix: **no**

Chaining outputs after a parenthesis is not supported

### FVSYN

Severity: **error** · Auto-fix: **no**

Invalid function argument syntax

### FVACI

Severity: **error** · Auto-fix: **no**

Name-value arguments in cell indexing not supported

### FVACS

Severity: **error** · Auto-fix: **no**

Quoted string used as name in `name=value` syntax

### FVAMI

Severity: **error** · Auto-fix: **no**

Name in `name=value` syntax is not a valid identifier

### UNSET

Severity: **error** · Auto-fix: **no**

Invalid use of an operator on the left side of an assignment

### LHROW

Severity: **error** · Auto-fix: **no**

Assignment left side cannot have multiple rows (`;`)

### RESWD

Severity: **error** · Auto-fix: **no**

Invalid use of a reserved word

### SYNEND

Severity: **error** · Auto-fix: **no**

Invalid use of the END operator

### MCPLD

Severity: **error** · Auto-fix: **no**

Invalid property syntax

### BADNOT

Severity: **error** · Auto-fix: **no**

Using `~` to ignore a value is not permitted

### BADNOTLHS

Severity: **error** · Auto-fix: **no**

Invalid use of logical not (`~`) on the left side

### STRIN

Severity: **error** · Auto-fix: **no**

A quoted character vector is unterminated

### DOUQT

Severity: **error** · Auto-fix: **no**

A double-quoted string is unterminated

### INBLK

Severity: **error** · Auto-fix: **no**

A block comment is unterminated at end of file

### BADFP

Severity: **error** · Auto-fix: **no**

Invalid floating-point constant (e.g. truncated `1.2.3`)

### BADHBH

Severity: **error** · Auto-fix: **no**

Invalid digit in a hexadecimal literal

### BADHBB

Severity: **error** · Auto-fix: **no**

Invalid digit in a binary literal

### BADHBHT

Severity: **error** · Auto-fix: **no**

Hex literal has too many digits for its type suffix

### BADHBBT

Severity: **error** · Auto-fix: **no**

Binary literal has too many digits for its type suffix

### HEXTOOLONG

Severity: **error** · Auto-fix: **no**

Hex literal has too many digits (max 16 without suffix)

### BINARYTOOLONG

Severity: **error** · Auto-fix: **no**

Binary literal has too many digits (max 64 without suffix)

### VTPOD

Severity: **error** · Auto-fix: **no**

Specify validation in order: size, then class, then functions

## Automatic fixes

Two checks carry automatic fixes:

- `BADNE` rewrites `!=` to `~=`, the MATLAB not-equal operator.
- `SEPEXR` inserts a `;` between two statements that share a line with no
  separator.

## Examples

### Incorrect

```matlab
x = 1 != 2;      % BADNE: `!=` is not MATLAB
a = 1 b = 2;     % SEPEXR: statements on one line without separator
if x > 0
    y = 'unterminated;   % STRIN
```

### Correct

```matlab
x = 1 ~= 2;
a = 1; b = 2;
if x > 0
    y = 'terminated';
end
```

### Fixed

```diff
- x = 1 != 2;
+ x = 1 ~= 2;
- a = 1 b = 2;
+ a = 1; b = 2;
```

## Configuration

```toml
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = []
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
