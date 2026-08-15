---
icon: lucide/check-circle
---

# Good Practices

**Default severity:** Warning
**Auto-fix:** Yes
**Category:** Good Practices
**Can be disabled:** Yes

## What this rule does

Encourages recommended coding practices for MATLAB code. All 106 checks are
handled by a single hybrid engine (`GoodPracticesEngine`). Node-level checks
cover simple pattern matches (error handling, `eval` usage, string
comparisons, parfor/spmd usage, redundant comparisons), while file-level
checks use metadata extraction and the symbol table for context-aware
analysis (last-statement detection, variable shadowing, class and property
validation). Each diagnostic carries the specific check ID (e.g. `TRYNC`,
`EVLCS`, `PFRNI`).

The checks span error handling, string and comparison idioms, `eval`/dynamic
code, parfor/spmd parallel practices, structure and field access, OOP and
class properties, logical-usage patterns, shared variables, arity, and App
Designer methods.

## Check IDs

### TRYNC

Severity: **warning** · Auto-fix: **no**

`try` without `catch`

### CTCH

Severity: **warning** · Auto-fix: **no**

`catch` block is empty

### WLAST

Severity: **warning** · Auto-fix: **no**

`warning` called as last statement in function

### WNTAG

Severity: **warning** · Auto-fix: **no**

`warning` without message ID

### ERTAG

Severity: **warning** · Auto-fix: **no**

`error` without message ID

### MEXCEP

Severity: **warning** · Auto-fix: **no**

`catch` without exception variable

### STCMP

Severity: **warning** · Auto-fix: **no**

Use `strcmp`/`strcmpi` instead of `==` for strings

### STCI

Severity: **warning** · Auto-fix: **no**

Use `strcmpi` for case-insensitive comparison

### STISA

Severity: **warning** · Auto-fix: **no**

Use `isa` instead of `class` + `strcmp`

### STRNU

Severity: **warning** · Auto-fix: **no**

Use `str2double` instead of `str2num`

### EVLCS

Severity: **warning** · Auto-fix: **no**

Avoid `eval`

### EVLDOT

Severity: **warning** · Auto-fix: **no**

Avoid `eval` for dynamic field access

### EVLEQ

Severity: **warning** · Auto-fix: **no**

Avoid `eval` for dynamic variable creation

### EVLSYS

Severity: **warning** · Auto-fix: **no**

Avoid `eval` for system commands

### EVLDUAL

Severity: **warning** · Auto-fix: **no**

Avoid `evalin`

### EVLSEQVAR

Severity: **warning** · Auto-fix: **no**

Avoid `eval` to create sequential variables

### NOANS

Severity: **warning** · Auto-fix: **no**

Statement result assigned to `ans`

### LOAD

Severity: **warning** · Auto-fix: **no**

`load` without output variable

### SEPEX

Severity: **info** · Auto-fix: **no**

Multiple statements on one line

### NBRAK1

Severity: **info** · Auto-fix: **yes**

Unnecessary brackets around scalar

### LNGNM

Severity: **warning** · Auto-fix: **no**

Variable name exceeds length

### CHAIN

Severity: **info** · Auto-fix: **no**

Method chaining on one line

### DISPLAY

Severity: **warning** · Auto-fix: **no**

Override `display` is discouraged

### FNDEF

Severity: **warning** · Auto-fix: **no**

Function not defined at expected location

### NOIN

Severity: **info** · Auto-fix: **no**

Function has no input validation

### VALST

Severity: **info** · Auto-fix: **no**

Validate function arguments

### PROP

Severity: **info** · Auto-fix: **no**

Property validation missing

### CPROP

Severity: **info** · Auto-fix: **no**

Constant property could be method

### FVAL

Severity: **warning** · Auto-fix: **no**

Function value not used

### FNCOLND

Severity: **warning** · Auto-fix: **no**

`end` used as column index without dimension

### COMNC

Severity: **info** · Auto-fix: **yes**

Comment lacks space after `%`

### ITERS

Severity: **warning** · Auto-fix: **no**

Loop variable shadows outer variable

### LOGPROD

Severity: **warning** · Auto-fix: **no**

Use `all` instead of `prod` on logical

### LOGMIN

Severity: **warning** · Auto-fix: **no**

Use `all` instead of `min` on logical

### LOGMAX

Severity: **warning** · Auto-fix: **no**

Use `any` instead of `max` on logical

### ELARLOG

Severity: **warning** · Auto-fix: **no**

Element-wise `&`/`\|` on logicals in if/while

### SHOCIRAA

Severity: **warning** · Auto-fix: **no**

Short-circuit in array context

### UNRPWR

Severity: **warning** · Auto-fix: **no**

Power of negative base may be complex

### ADAPPREF

Severity: **warning** · Auto-fix: **no**

Avoid `addpref` (use settings)

### KEYBOARDFUN

Severity: **warning** · Auto-fix: **no**

`keyboard` left in code

### GVMIS

Severity: **warning** · Auto-fix: **no**

Global variable used but never declared

### PFEVB

Severity: **warning** · Auto-fix: **no**

EVALIN('base')/ASSIGNIN('base') inside a PARFOR loop refers to worker base workspace

### PFGP

Severity: **warning** · Auto-fix: **no**

Assigning to GLOBAL/PERSISTENT variable inside a PARFOR loop

### PFGV

Severity: **warning** · Auto-fix: **no**

Using a GLOBAL variable in a PARFOR loop

### PFIIN

Severity: **warning** · Auto-fix: **no**

The input variable should be initialized before the PARFOR loop

### PFOUS

Severity: **warning** · Auto-fix: **no**

The output variable might not be used after the PARFOR loop

### PFRNI

Severity: **warning** · Auto-fix: **yes**

Explicit increment in a PARFOR loop; parfor only supports an increment of one

### PFRIN

Severity: **warning** · Auto-fix: **no**

The reduction variable might not be set before the PARFOR loop

### PFRUS

Severity: **warning** · Auto-fix: **no**

The reduction variable might not be used after the PARFOR loop

### PFTUSW

Severity: **warning** · Auto-fix: **no**

The temporary variable might be used after the PARFOR loop

### PFUIXW

Severity: **warning** · Auto-fix: **no**

The index variable might be used after the PARFOR loop

### SPEVB

Severity: **warning** · Auto-fix: **no**

EVALIN('base')/ASSIGNIN('base') inside an SPMD block refers to worker base workspace

### SPGV

Severity: **warning** · Auto-fix: **no**

GLOBAL/PERSISTENT variable in an SPMD block might fail on a worker

### DSPMDA

Severity: **warning** · Auto-fix: **no**

Distributed array must be created outside of an SPMD block

### COMFS

Severity: **warning** · Auto-fix: **no**

Comma makes the file a script, so functions are local

### DUALC

Severity: **warning** · Auto-fix: **no**

Command might be prematurely ended by comma

### RMFLD

Severity: **warning** · Auto-fix: **no**

`rmfield` output must be assigned back to the structure

### RMWRN

Severity: **warning** · Auto-fix: **no**

Warning tag has been removed from MATLAB

### SEMFS

Severity: **warning** · Auto-fix: **no**

Semicolon makes the file a script, so functions are local

### STFLD

Severity: **warning** · Auto-fix: **no**

`setfield` output must be assigned back to the structure

### STRSZ

Severity: **warning** · Auto-fix: **no**

Use `strcmp` to compare character vectors of different sizes

### ATTF

Severity: **warning** · Auto-fix: **no**

Unable to determine if the `Abstract` attribute expression is true or false

### ATTOF

Severity: **info** · Auto-fix: **no**

Setting the class attribute `Abstract` to false is not recommended

### MCPO

Severity: **warning** · Auto-fix: **no**

`SetObservable`/`GetObservable`/`AbortSet` property has no effect in a value class

### MCSAC

Severity: **warning** · Auto-fix: **no**

`SetAccess` cannot be set on Constant properties

### MOBSRV

Severity: **info** · Auto-fix: **no**

`SetObservable`/`GetObservable` on a Constant property has no effect

### MDEPIN

Severity: **warning** · Auto-fix: **no**

Default values should not be assigned to dependent properties

### MCCPI

Severity: **warning** · Auto-fix: **no**

Initialize the Constant property or make it an Abstract Constant property

### MGMD

Severity: **warning** · Auto-fix: **no**

`get` method should be implemented for each dependent property without private `GetAccess`

### MCCPE

Severity: **warning** · Auto-fix: **no**

Attempting to call a property or event as a function

### MTHANS

Severity: **info** · Auto-fix: **no**

Using `ANS` as a method name is not recommended

### MHERM

Severity: **info** · Auto-fix: **no**

Parenthesize the multiplication of a variable and its transpose

### MNUML

Severity: **warning** · Auto-fix: **no**

Use `VAR_NAME(numel(...), numel(...))` to create a square matrix

### COMPNOP

Severity: **warning** · Auto-fix: **yes**

Comparison with `true` simplifies to the function call itself

### COMPNOT

Severity: **warning** · Auto-fix: **yes**

Comparison with `~= true` or `== false` simplifies to `~call(...)`

### M3COL

Severity: **warning** · Auto-fix: **no**

Three colons (`a:b:c:d`) in an expression is probably unintended

### BDLGI

Severity: **warning** · Auto-fix: **no**

Variable might be set by a nonlogical operator

### BDLOG1

Severity: **warning** · Auto-fix: **no**

Non-scalar logical value used in a conditional expression

### BDLOG2

Severity: **warning** · Auto-fix: **no**

Scalar non-logical value used in a conditional expression

### BDSCA

Severity: **warning** · Auto-fix: **no**

`&&`/`\|\|` used in a scalar context with a non-scalar operand

### BDSCI

Severity: **warning** · Auto-fix: **no**

Variable might be set by a nonscalar operator

### MCHDP

Severity: **warning** · Auto-fix: **no**

Property default that directly constructs a handle is shared by all instances

### MCHDT

Severity: **warning** · Auto-fix: **no**

Property default that resolves to a handle is shared by all instances

### SHVAU

Severity: **warning** · Auto-fix: **no**

Ambiguous shared-variable usage between a nested function and its parent

### GTARG

Severity: **warning** · Auto-fix: **no**

Function might be called with too many arguments

### LTARG

Severity: **warning** · Auto-fix: **no**

Function might be called with too few arguments

### CTPCT

Severity: **warning** · Auto-fix: **no**

`sprintf`/`fprintf` format might not agree with the argument count

### FXSET

Severity: **warning** · Auto-fix: **no**

Loop index variable is changed inside of a `for` loop

### SIMPT

Severity: **warning** · Auto-fix: **no**

`import` statement does not run first in a function

### TLEV

Severity: **warning** · Auto-fix: **no**

Dynamic-code function used as a sub-expression, not a top-level statement

### UNONC

Severity: **warning** · Auto-fix: **no**

`onCleanup` output must be assigned to a variable, not `~`

### MIPC1

Severity: **warning** · Auto-fix: **no**

`computer('arch')` is platform-specific

### SUBSINDEX

Severity: **warning** · Auto-fix: **no**

Do not overload `subsindex` for fundamental data types

### VTFIN

Severity: **warning** · Auto-fix: **no**

Validated value should be the first input to a `validate*` function

### CTOINW

Severity: **warning** · Auto-fix: **no**

Constructed object passed to its own constructor

### FXUP

Severity: **warning** · Auto-fix: **no**

Outer loop index set inside a nested function

### ADMTHDINV

Severity: **warning** · Auto-fix: **no**

Class method called without `app` as the first argument

### ADPROP

Severity: **warning** · Auto-fix: **no**

Property assigned through a bare identifier instead of `app.PROP`

### ADPROPLC

Severity: **warning** · Auto-fix: **no**

Property read through a bare identifier instead of `app.PROP`

### MCNPN

Severity: **warning** · Auto-fix: **no**

Member access on the object that is not declared in the class

### MCNPR

Severity: **warning** · Auto-fix: **no**

Assignment target on the object that is not a property

### MCSNOV

Severity: **warning** · Auto-fix: **no**

Value-class setter does not return the modified object

### MCSOH

Severity: **warning** · Auto-fix: **no**

Handle-class setter unnecessarily returns the modified object

### MCVM

Severity: **warning** · Auto-fix: **no**

Value-class method modifying the object has no output

### MCCSPS

Severity: **warning** · Auto-fix: **no**

Constant property name used as a struct in a dot-access chain

### MCSUP

Severity: **warning** · Auto-fix: **no**

Setter accesses a property other than the one it sets

## Automatic fixes

Rewrites the flagged construct into the cleaner equivalent:

- `call(...) == true` → `call(...)` (COMPNOP) and
  `call(...) ~= true` / `call(...) == false` → `~call(...)` (COMPNOT).
- `parfor i = 1:step:end` → `parfor i = 1:end` (PFRNI).
- `%comment` → `% comment` (COMNC).
- `(scalar)` → `scalar` (NBRAK1).

## Examples

### Incorrect

```matlab
if isa(x, 'double') == true      % COMPNOP
    disp('double');
end
parfor i = 1:2:10                % PFRNI
    y(i) = i;
end
%comment with no space           % COMNC
x = (5);                         % NBRAK1
```

### Correct

```matlab
if isa(x, 'double')              % COMPNOP fix applied
    disp('double');
end
parfor i = 1:10                  % PFRNI fix applied
    y(i) = i;
end
% comment with space             % COMNC fix applied
x = 5;                           % NBRAK1 fix applied
```

### Fixed

```diff
- if isa(x, 'double') == true
+ if isa(x, 'double')
- parfor i = 1:2:10
+ parfor i = 1:10
```

## Configuration

```toml
[lint.rules.GOOD_PRACTICES_ENGINE]
severity = "warning"
max_variable_name_length = 63
disabled_checks = []
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
