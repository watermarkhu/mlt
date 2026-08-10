---
icon: lucide/check-circle
---

# Good Practices

**Default severity:** Warning
**Auto-fix:** No
**Category:** Good Practices
**Can be disabled:** Yes

## What these checks do

These checks cover function-call conventions: `sprintf`/`fprintf` format strings that do not agree with the argument count, `for` loop iterator variables that are reassigned inside the loop, `import` statements that do not run first in a function, dynamic-code functions (`eval`, `evalc`, `evalin`, `feval`) used as sub-expressions, `onCleanup` outputs that are discarded or assigned to `~`, and the platform-specific `computer('arch')` query.

## Check IDs

| Check ID | Message |
| -------- | ------- |
| `CTPCT` | The format might not agree with the argument count. |
| `FXSET` | Loop index `VAR_NAME` is changed inside of a FOR loop. |
| `SIMPT` | This import statement runs before any other code in function `VAR_NAME`. Consider placing it at the top of the function body. |
| `TLEV` | `VAR_NAME` could be very inefficient unless it is a top-level statement in its function. |
| `UNONC` | Assign the onCleanup output argument to a variable. Do not use the tilde operator (~) in place of a variable. |
| `MIPC1` | Calling the computer function with 'arch' returns 'win64', 'glnxa64', or 'maci64'. |
| `ATTF` | The Code Analyzer is unable to determine if the expression assigned to the `Abstract` attribute evaluates to true or false. |
| `ATTOF` | Setting the class attribute `Abstract` to false is not recommended. |
| `MCPO` | `SetObservable`/`GetObservable`/`AbortSet` property has no effect in a value class. |
| `MCSAC` | `SetAccess` cannot be set on Constant properties. |
| `MOBSRV` | `SetObservable`/`GetObservable` on a Constant property has no effect. |
| `MDEPIN` | Default values should not be assigned to dependent properties. |
| `MCCPI` | Initialize the Constant property or make it an Abstract Constant property. |
| `MGMD` | `get` method should be implemented for each dependent property without private `GetAccess`. |
| `MCCPE` | Attempting to call a property or event as a function. |
| `MTHANS` | Using `ANS` as a method name is not recommended. |
| `MHERM` | Parenthesize the multiplication of a variable and its transpose. |
| `MNUML` | To create a square matrix, use `VAR_NAME(numel(...), numel(...))`. |
| `COMFS` | This comma makes the file a script. Therefore, all functions in the file are local functions. |
| `DUALC` | Command might be prematurely ended by comma. |
| `RMFLD` | RMFIELD output must be assigned back to the structure. |
| `RMWRN` | The warning with tag VAR_NAME has been removed from MATLAB, so this statement has no effect. |
| `SEMFS` | This semicolon makes the file a script. Therefore, all functions in the file are local functions. |
| `STFLD` | SETFIELD output must be assigned back to the structure. |
| `STRSZ` | Use STRCMP to compare character vectors that can have different sizes. |
| `SUBSINDEX` | Do not overload 'subsindex' for fundamental data types. |
| `VTFIN` | VAR_NAME should be the first input argument to the VAR_NAME function. |
| `CTOINW` | Use of constructed object as input to constructor is not necessary. |
| `FXUP` | Outer loop index VAR_NAME is set inside a nested function. |

### App Designer / OOP practice

| Check ID | Message |
| -------- | ------- |
| `ADMTHDINV` | Use VAR_NAME(app, ...) to call this function. |
| `ADPROP` | Use app.VAR_NAME to refer to this property. |
| `ADPROPLC` | Use app.VAR_NAME to reference a property of app. |
| `MCNPN` | VAR_NAME is referenced but is not a property, method, or event name defined in this class. |
| `MCNPR` | VAR_NAME is not a property, but is the target of an assignment. |
| `MCSNOV` | Set function in value class must return the modified object. |
| `MCSOH` | Set function in handle class does not need to return the modified object. |
| `MCVM` | Value class method that modifies the object must return the modified object. |
| `MCCSPS` | Constant property VAR_NAME is not modified. 'VAR_NAME.VAR_NAME' creates a struct named VAR_NAME with a field named VAR_NAME. |
| `MCSUP` | The set method for the property VAR_NAME should not access another property (VAR_NAME). |

### Logical / comparison / range

| Check ID | Message |
| -------- | ------- |
| `COMPNOP` | This logical comparison simplifies to VAR_NAME(...). Did you mean to use VAR_NAME to evaluate function argument: VAR_NAME(...VAR_NAME...)? |
| `COMPNOT` | This logical comparison simplifies to ~VAR_NAME(...). Did you mean to use VAR_NAME to evaluate function argument: VAR_NAME(...VAR_NAME...)? |
| `M3COL` | Using three colons (a:b:c:d) in an expression is probably unintended. |

### Logical usage / handle defaults / shared variables / arity

| Check ID | Message |
| -------- | ------- |
| `BDLGI` | Variable might be set by a nonlogical operator. |
| `BDLOG1` | A scalar logical value is expected in the conditional expression. Use 'any' or 'all' to reduce the array to a logical scalar. |
| `BDLOG2` | A scalar logical value is expected in the conditional expression. Use 'any' or 'all' to reduce the array to a logical scalar, or compare the scalar value to 0. |
| `BDSCA` | Unexpected use of VAR_OPERATOR in a scalar context. |
| `BDSCI` | Variable might be set by a nonscalar operator. |
| `MCHDP` | A property default value that is a handle will cause all instances to share the same object data. To avoid sharing, create the property value in the constructor. For intentional sharing, consider using a Constant property. |
| `MCHDT` | Declaring the value of a property as a handle might cause all instances to share the same default handle. To avoid sharing, create the handle for this property in the constructor. To express that sharing is intentional, use the Constant property attribute. |
| `SHVAU` | Confusing usage of name VAR_NAME on lines VAR_NUMBER and VAR_NUMBER. Initialize VAR_NAME before line VAR_NUMBER to make it a shared variable or rename VAR_NAME on line VAR_NUMBER to disambiguate. |
| `GTARG` | Function might be called with too many arguments. |
| `LTARG` | Function might be called with too few arguments. |

### Parfor / SPMD / parallel practices

| Check ID | Message |
| -------- | ------- |
| `PFEVB` | Using EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop refers to the worker machines' base workspaces. |
| `PFGP` | Avoid assigning to GLOBAL or PERSISTENT variable `VAR_NAME` inside a PARFOR loop. |
| `PFGV` | Avoid using GLOBAL variable `VAR_NAME` in a PARFOR loop. |
| `PFIIN` | The input variable `VAR_NAME` should be initialized before the PARFOR loop. |
| `PFOUS` | The output variable `VAR_NAME` might not be used after the PARFOR loop. |
| `PFRNI` | Do not specify the increment explicitly. The parfor loop can only use an increment of one. |
| `PFRIN` | The reduction variable `VAR_NAME` might not be set before the PARFOR loop. |
| `PFRUS` | The reduction variable `VAR_NAME` might not be used after the PARFOR loop. |
| `PFTUSW` | The temporary variable `VAR_NAME` might be used after the PARFOR loop on line `VAR_NUMBER`. |
| `PFUIXW` | The index variable `VAR_NAME` might be used after the PARFOR loop on line `VAR_NUMBER`. |
| `SPEVB` | Using EVALIN('base') or ASSIGNIN('base') inside an SPMD block refers to the worker machines' base workspaces. |
| `SPGV` | Using the GLOBAL or PERSISTENT variable `VAR_NAME` in an SPMD block might fail because it is accessed on a worker machine. |
| `DSPMDA` | Distributed array must be created outside of an SPMD block. |

## CTPCT - Format and Argument Count

Flags `sprintf` and `fprintf` calls whose format string contains a number of conversion specifiers that does not match the number of remaining arguments. Escaped `%%`, `%*` width specifiers, and `%n$` positional specifiers are not counted.

### Why this matters

A mismatch between the format string and the arguments produces incorrect output or runtime errors (`sprintf` returns fewer/more values than expected, or conversion fails).

### Examples

#### Incorrect

```matlab
fprintf('%d %d', x);       % two specifiers, one argument
sprintf('%s %s', a);       % two specifiers, one argument
```

#### Correct

```matlab
fprintf('%d %d', x, y);    % two specifiers, two arguments
sprintf('100%% %d', x);    % %% is escaped, one specifier
```

### Limitations

`%*` (width from argument) and `%n$` (positional) specifiers are not counted. A format string with a file identifier as the first argument (`fprintf(fid, ...)`) is supported: the first string argument is treated as the format.

## FXSET - Loop Index Modified Inside the Loop

Flags `for` loops whose iterator variable is assigned inside the loop body.

### Why this matters

Changing the loop index inside the body changes the iteration sequence in ways that are usually unintended and hard to debug. MATLAB's `for` loop index should only be updated by the loop itself.

### Examples

#### Incorrect

```matlab
for i = 1:10
    i = 5;                 % modifies the loop index
end
```

#### Correct

```matlab
for i = 1:10
    y = i;                 % reads the loop index
end
```

## SIMPT - Import Not First in Function

Flags `import` commands that do not run before any other code in their enclosing function.

### Why this matters

An `import` that executes after other statements may not affect the earlier code, and the imported symbols may not be available where the user expects them. MATLAB's Code Analyzer recommends placing `import` statements at the top of the function body.

### Examples

#### Incorrect

```matlab
function f()
    x = 1;
    import foo.bar
end
```

#### Correct

```matlab
function f()
    import foo.bar;
    x = 1;
end
```

Imports at the file/script level are not flagged.

## TLEV - Dynamic-Code Function Used as a Sub-Expression

Flags `eval`, `evalc`, `evalin`, and `feval` calls that are used as sub-expressions inside a larger expression instead of as top-level statements.

### Why this matters

Dynamic-code execution is expensive. Using these functions in the middle of an expression forces the interpreter to construct and evaluate code at runtime while the surrounding expression waits; a top-level statement can often be replaced by a more efficient, static construct.

### Examples

#### Incorrect

```matlab
eval(x + y);               % bare statement; result discarded
z = f(eval(y));            % eval nested inside another call
z = eval(y) + 1;           % eval nested inside an operator
```

#### Correct

```matlab
z = eval(y);               % eval is the entire right-hand side
```

A call that is the entire right-hand side of an assignment is treated as a top-level statement and is not flagged.

## UNONC - onCleanup Output Not Assigned

Flags `onCleanup` calls whose output object is discarded or assigned to `~`.

### Why this matters

The object returned by `onCleanup` runs its cleanup function when it is destroyed. If the object is discarded immediately or assigned to `~`, the cleanup runs at the end of the statement and the function never executes at the intended scope.

### Examples

#### Incorrect

```matlab
onCleanup(@myCleanup);     % object discarded immediately
[~] = onCleanup(@myCleanup); % cleanup runs right away
```

#### Correct

```matlab
cleanupObj = onCleanup(@myCleanup);
```

## MIPC1 - computer('arch')

Flags calls to `computer` with a single string argument `'arch'`.

### Why this matters

`computer('arch')` returns one of `'win64'`, `'glnxa64'`, or `'maci64'` and is often used in code that must remain platform-specific. The check points out the platform-dependent result so you can decide whether the behavior is intended.

### Examples

#### Incorrect

```matlab
arch = computer('arch');
```

#### Correct

```matlab
c = computer;              % no 'arch' argument
arch = computer('win');    % not the 'arch' query
```

## OOP / class / property checks

These checks analyze `classdef` files: class-level attributes, property block attributes, dependent/constant property semantics, and method/property naming.

### ATTF / ATTOF - Class Abstract Attribute

`ATTF` fires when the expression assigned to the class `Abstract` attribute cannot be determined to be a boolean literal. `ATTOF` fires when `Abstract` is explicitly set to `false`.

### Why this matters

The Code Analyzer cannot always statically evaluate the value assigned to `Abstract`; a non-literal value may be neither `true` nor `false` at parse time. Setting `Abstract` to `false` is redundant and often masks a mistaken `Abstract` declaration.

### Examples

#### Incorrect

```matlab
classdef (Abstract = someVar) Foo      % ATTF: value is not a boolean literal
end

classdef (Abstract = false) Foo        % ATTOF: explicit false is not recommended
end
```

#### Correct

```matlab
classdef (Abstract = true) Foo
end
```

### MCPO - Observable Attributes on a Value Class

Fires when a property block of a value class (a class that does not inherit from `handle`) uses `SetObservable`, `GetObservable`, or `AbortSet`.

### Why this matters

Observable attributes only take effect on handle classes. On a value class, listeners cannot observe property changes, so the attributes are silently ignored.

### Examples

#### Incorrect

```matlab
classdef ValClass
    properties (SetObservable)
        Data
    end
end
```

#### Correct

```matlab
classdef HClass < handle
    properties (SetObservable)
        Data
    end
end
```

### MCSAC - SetAccess on Constant Properties

Fires when a property block declares both `Constant` and `SetAccess`.

### Why this matters

Constant properties are set only at class definition time; `SetAccess` cannot control writes to them and is meaningless.

### Examples

#### Incorrect

```matlab
classdef MyConst
    properties (Constant, SetAccess = public)
        X = 1
    end
end
```

#### Correct

```matlab
classdef MyConst
    properties (Constant)
        X = 1
    end
end
```

### MOBSRV - Observable Attributes on Constant Properties

Fires when a property block declares both `Constant` and `SetObservable` or `GetObservable`.

### Why this matters

Constant properties are evaluated once at class load; observable accessors have no effect on them.

### Examples

#### Incorrect

```matlab
classdef MyConst
    properties (Constant, SetObservable)
        X = 1
    end
end
```

#### Correct

```matlab
classdef MyConst
    properties (Constant)
        X = 1
    end
end
```

### MDEPIN - Default Values on Dependent Properties

Fires when a `Dependent` property is assigned a default value.

### Why this matters

Dependent properties do not store values; they are computed by `get` methods on demand. A default value would never be stored or returned.

### Examples

#### Incorrect

```matlab
classdef DepClass
    properties (Dependent)
        Y = 5
    end
end
```

#### Correct

```matlab
classdef DepClass
    properties (Dependent)
        Y
    end
end
```

### MCCPI - Uninitialized Constant Property

Fires when a `Constant` property has no default value and is not declared `Abstract`.

### Why this matters

A constant property that is never initialized cannot be read. Initialize it, or declare the property (or class) `Abstract`.

### Examples

#### Incorrect

```matlab
classdef MyConst
    properties (Constant)
        X
    end
end
```

#### Correct

```matlab
classdef MyConst
    properties (Constant)
        X = 1
    end
end
```

```matlab
classdef MyConst
    properties (Constant, Abstract)
        X
    end
end
```

### MGMD - Dependent Property Without a get Method

Fires for each `Dependent` property that has no corresponding `get.PropName` method, unless the block declares `GetAccess = private`.

### Why this matters

A dependent property is computed by its `get` method. Without one, the property cannot return a value.

### Examples

#### Incorrect

```matlab
classdef DepClass
    properties (Dependent)
        Y
    end
end
```

#### Correct

```matlab
classdef DepClass
    properties (Dependent)
        Y
    end
    methods
        function val = get.Y(obj)
            val = 1;
        end
    end
end
```

### MCCPE - Property or Event Called as a Function

Fires when a `function_call` inside the class body uses the name of a property or event defined by the class.

### Why this matters

Properties and events are not callable. The call is likely a mistake — the code probably meant to index the property or reference the object field.

### Examples

#### Incorrect

```matlab
classdef Foo
    properties
        Color
    end
    methods
        function go(obj)
            y = Color(1);        % property called as a function
        end
    end
end
```

#### Correct

```matlab
classdef Foo
    properties
        Color
    end
    methods
        function go(obj)
            y = obj.Color;       % field access
        end
    end
end
```

### MTHANS - ans as a Method Name

Fires when a method is named `ans`.

### Why this matters

`ans` is the implicit output variable in MATLAB and is frequently overwritten by the interpreter. A method named `ans` is error-prone and confusing.

### Examples

#### Incorrect

```matlab
classdef Foo
    methods
        function ans = ans(obj)
            ans = 1;
        end
    end
end
```

#### Correct

```matlab
classdef Foo
    methods
        function result = compute(obj)
            result = 1;
        end
    end
end
```

### MHERM - Unparenthesized Multiplication by a Transpose

Fires when a `*` or `.*` multiplication has a transposed operand (`'`) and the multiplication is not wrapped in parentheses.

### Why this matters

`x * x'` is not guaranteed to be Hermitian in floating-point arithmetic; parenthesizing as `(x * x')` makes the intent explicit and ensures a Hermitian result.

### Examples

#### Incorrect

```matlab
z = x * x';
```

#### Correct

```matlab
z = (x * x');
```

### MNUML - Square Matrix Creation With a Single numel Argument

Fires when `zeros`, `ones`, `rand`, `randn`, `false`, or `true` is called with a single `numel(...)` argument.

### Why this matters

`zeros(numel(x))` creates a square matrix with `numel(x)` rows and columns, which is rarely the intended shape. Pass both dimensions (`zeros(numel(x), numel(x))`) or use `size` (`zeros(size(x))`) to make the intent explicit.

### Examples

#### Incorrect

```matlab
y = zeros(numel(x));
```

#### Correct

```matlab
y = zeros(numel(x), numel(x));
y = zeros(size(x));
```

## Parfor / SPMD / parallel practices

These checks analyze `parfor` loops and `spmd` blocks. `parfor` is a `for_statement` whose source text starts with `parfor`; `spmd` is an `spmd_statement` node.

### PFRNI - Explicit Increment in a PARFOR Loop

Fires when a `parfor` range is written as `start:step:end` (three parts). `parfor` only supports an increment of one.

#### Incorrect

```matlab
parfor i = 1:2:10
    x(i) = i;
end
```

#### Correct

```matlab
parfor i = 1:10
    x(i) = i;
end
```

This check provides an automatic fix that rewrites `1:2:10` as `1:10`.

### PFEVB / SPEVB - EVALIN/ASSIGNIN('base') in PARFOR or SPMD

Fires when `evalin` or `assignin` is called with `'base'` as the first argument inside a `parfor` loop body (`PFEVB`) or an `spmd` block (`SPEVB`). Calls with other workspaces (e.g., `'caller'`) are not flagged.

#### Incorrect

```matlab
parfor i = 1:10
    evalin('base', 'x');
end

spmd
    assignin('base', 'y', 1);
end
```

#### Correct

```matlab
parfor i = 1:10
    evalin('caller', 'x');
end
```

### PFGP - Assigning to a GLOBAL/PERSISTENT Variable in PARFOR

Fires when a `parfor` body assigns to a variable declared `global` or `persistent` anywhere in the file.

#### Incorrect

```matlab
global gVar;
parfor i = 1:10
    gVar = i;
end
```

#### Correct

```matlab
parfor i = 1:10
    x(i) = i;
end
```

### PFGV - Using a GLOBAL Variable in PARFOR

Fires when a `parfor` body reads a variable declared `global`. (Assignment to a global is handled by `PFGP`.)

#### Incorrect

```matlab
global gVar;
parfor i = 1:10
    x(i) = gVar;
end
```

### SPGV - Using a GLOBAL/PERSISTENT Variable in SPMD

Fires when an `spmd` block uses a variable declared `global` or `persistent`. Each worker machine has its own copy of the workspace, so the value is undefined.

#### Incorrect

```matlab
global g2;
spmd
    z = g2 + 1;
end
```

### DSPMDA - Distributed Array Created Inside SPMD

Fires when `distributed`, `gpuArray`, or `codistributed` is called inside an `spmd` block. Distributed arrays must be created on the client.

#### Incorrect

```matlab
spmd
    d = distributed(zeros(100));
end
```

#### Correct

```matlab
d = distributed(zeros(100));
spmd
    work_with(d);
end
```

### PFIIN - Input Variable Not Initialized Before PARFOR

Fires when a variable is read inside a `parfor` body but is never initialized before the loop and is not assigned inside the loop. Function inputs and `global`/`persistent` declarations count as initialized.

#### Incorrect

```matlab
parfor i = 1:10
    q = z + i;      % z is never initialized
end
```

#### Correct

```matlab
z = 0;
parfor i = 1:10
    q = z + i;
end
```

### PFOUS - Output Variable Not Used After PARFOR

Fires when a simple variable (not `x(i)` indexed assignment) is assigned inside a `parfor` body but never read after the loop.

#### Incorrect

```matlab
parfor i = 1:10
    q = i;
end
```

#### Correct

```matlab
parfor i = 1:10
    q = i;
end
disp(q);
```

### PFTUSW - Temporary Variable Used After PARFOR

Fires when a simple variable assigned inside a `parfor` body (other than the index variable) is read after the loop. The message reports the line of the first use after the loop; the value comes from an unspecified worker.

#### Incorrect

```matlab
parfor i = 1:10
    tmp = compute(i);
    x(i) = tmp;
end
disp(tmp);
```

### PFUIXW - Index Variable Used After PARFOR

Fires when the `parfor` index variable is read after the loop. The message reports the line of the first use after the loop. Uses inside a subsequent `for`/`parfor` body are ignored because the variable is re-bound there.

#### Incorrect

```matlab
parfor i = 1:10
    x(i) = i;
end
disp(i);
```

## Structure, String, and Misc Checks

These checks cover structure-manipulation calls whose results are silently discarded, character-vector comparisons that should use `strcmp`, warning tags that no longer exist, command statements that a comma may prematurely end, and file-level separators that turn a function file into a script file.

### RMFLD / STFLD - Structure Output Must Be Assigned Back

Fires when `rmfield` or `setfield` is called as a bare statement instead of being assigned back to the structure.

### Why this matters

`rmfield` and `setfield` return the modified structure; calling them as a bare statement discards the result, so the modification never takes effect.

### Examples

#### Incorrect

```matlab
rmfield(s, 'a');       % result discarded
setfield(s, 'a', 1);   % result discarded
```

#### Correct

```matlab
s = rmfield(s, 'a');
s = setfield(s, 'a', 1);
```

### STRSZ - Comparing Character Vectors of Different Sizes

Fires when `==` or `~=` compares two string literals of different lengths.

### Why this matters

`==` on two character vectors performs element-wise comparison; when the vectors have different sizes the result is not a scalar logical, which is almost always a bug. `strcmp` compares the whole vectors.

### Examples

#### Incorrect

```matlab
x = 'abc' == 'abcd';
```

#### Correct

```matlab
x = strcmp('abc', 'abcd');
```

### RMWRN - Removed Warning Tag

Fires when `warning` is called with a message ID tag that MATLAB has removed. The denylist of removed tags is currently **empty** (placeholder), so the check is inert until tags are populated.

### Why this matters

A `warning(...)` call with a removed message ID tag silently does nothing, which hides genuine warnings.

### DUALC - Command Prematurely Ended by Comma

Fires when a command-syntax statement is immediately followed by a comma.

### Why this matters

In command syntax, the command consumes the rest of the line; a comma after a command may end it prematurely and make the remaining text a separate statement.

### Examples

#### Incorrect

```matlab
disp hello, disp world
```

#### Correct

```matlab
disp hello;
disp world;
```

### Limitations

`DUALC` is a heuristic: it fires whenever a `command` node is immediately followed by a comma sibling. In one-line `if`/`for` constructions (e.g., `if x, disp y, end`) this also matches and is reported.

### COMFS / SEMFS - File Structure Makes Functions Local

Fires when a file that contains a `function` definition also has a top-level comma (`COMFS`) or semicolon (`SEMFS`) statement.

### Why this matters

A file that contains both script statements and `function` definitions is treated as a script, so every function becomes a local function.

### Examples

#### Incorrect (the comma and semicolon at the top level make `f` a local function)

```matlab
x = 1,
y = 2;
function f()
end
```

#### Correct (a pure function file)

```matlab
function f()
    x = 1;
    y = 2;
end
```

### Limitations

- **`RMWRN`** uses a denylist of removed warning tags that is currently **empty** (placeholder). The mechanism is implemented and will start reporting as tags are added to the denylist.
- **`STRSZ`** only compares two literal character vectors; comparisons involving variables or non-string operands are not reported.

## Logical, Comparison, and Range Checks

These checks flag redundant logical comparisons of function results and accidentally-chained colon expressions.

### COMPNOP - Comparison With `true` Simplifies to the Call

Fires when a `function_call` is compared with `== true` (in either order). The comparison always has the same value as the call itself.

#### Why this matters

`isa(x, 'double') == true` reads as if `isa` needed a second step to produce a logical, and obscures the fact that `isa` already returns a logical.

#### Incorrect

```matlab
if isa(x, 'double') == true
end
```

#### Correct

```matlab
if isa(x, 'double')
end
```

This check provides an automatic fix that replaces the whole comparison with the call: `isa(x, 'double') == true` becomes `isa(x, 'double')`.

### COMPNOT - Comparison With `~= true` or `== false` Simplifies to `~call(...)`

Fires when a `function_call` is compared with `~= true` or `== false` (in either order). The comparison always equals the negated call.

#### Why this matters

`~= true` and `== false` are indirect ways of writing logical negation and obscure the intent.

#### Incorrect

```matlab
if isa(x, 'double') ~= true
end
if isa(x, 'double') == false
end
```

#### Correct

```matlab
if ~isa(x, 'double')
end
```

This check provides an automatic fix that replaces the whole comparison with the negated call: `isa(x, 'double') == false` becomes `~isa(x, 'double')`.

The fix is semantically safe when the call returns a scalar logical, which is the intended use; it is offered unconditionally to match MATLAB's behavior.

### M3COL - Three Colons in an Expression

Fires when an expression contains three colons (`a:b:c:d`). MATLAB's colon operator accepts `start:end` or `start:step:end` only, so a third colon is almost always a typo.

#### Why this matters

`1:2:3:4` is not valid MATLAB — the extra `:` turns the statement into a syntax error.

#### Incorrect

```matlab
a = 1:2:3:4;
```

#### Correct

```matlab
a = 1:2:3;
```

This check does not provide an automatic fix.

## BDLGI - Variable Might Be Set by a Nonlogical Operator

Flags variables that are assigned from an arithmetic operator (`+`, `-`, `*`, `/`, `^`, `.*`, ...) and later used as a bare `if` / `while` condition. A numeric value used as a condition is almost always a mistake; the condition should be a logical expression.

### Why this matters

Using a computed numeric value as a condition relies on the implicit nonzero-is-true rule, which is easy to misread and often indicates a missing comparison (for example `if x` instead of `if x > 0`).

### Examples

#### Incorrect

```matlab
x = a + b;
if x
    ...
end
```

#### Correct

```matlab
x = a + b;
if x > 0
    ...
end
```

The check only fires when the type environment can confirm the variable is not logical.

## BDLOG1 - Non-Scalar Logical Value in a Conditional Expression

Flags `if` / `while` conditions that are logical but not scalar. MATLAB requires a scalar logical value in a conditional expression; an array logical condition is an error (or `all`/`any` was intended).

### Why this matters

A logical vector produced by a comparison (`x = a > b; if x`) does not give the expected single true/false answer. Use `any` or `all` to reduce it to a scalar.

### Examples

#### Incorrect

```matlab
x = a > b;      % logical vector when a, b are vectors
if x
    ...
end
```

#### Correct

```matlab
if any(a > b)
    ...
end
```

## BDLOG2 - Scalar Non-Logical Value in a Conditional Expression

Flags `if` / `while` conditions that are scalar but not provably logical, such as a numeric scalar literal or a variable assigned a scalar number.

### Why this matters

A scalar numeric condition (`if x` where `x = 5`) always evaluates to true. MATLAB recommends comparing the scalar to 0 (`if x ~= 0`) to make the intent explicit.

### Examples

#### Incorrect

```matlab
x = 5;
if x
    ...
end
```

#### Correct

```matlab
x = 5;
if x ~= 0
    ...
end
```

## BDSCA - Short-Circuit Operator in a Scalar Context

Flags `&&` / `||` operators whose operand is a non-scalar logical array. Short-circuit operators require scalar logical operands; element-wise `&` / `|` (or `any`/`all`) should be used for arrays.

### Why this matters

`&&` on an array is a runtime error. The fix is usually to reduce the operand with `any`/`all` or switch to the element-wise operator.

### Examples

#### Incorrect

```matlab
x = a > b;          % logical vector
if x && y
    ...
end
```

#### Correct

```matlab
if all(x) && y
    ...
end
```

## BDSCI - Variable Might Be Set by a Nonscalar Operator

Flags variables assigned from an array-producing expression (a colon range `a:b`, a matrix `[...]`, or a cell `{...}` literal with more than one element) and later used in a scalar context such as a bare `if` / `while` condition.

### Why this matters

Using an array where a scalar is expected is a common mistake; the array assignment usually indicates a different intent (for example, a missing subscript).

### Examples

#### Incorrect

```matlab
idx = 1:10;
if idx
    ...
end
```

#### Correct

```matlab
idx = 1:10;
if isempty(idx)
    ...
end
```

## MCHDP - Property Default Directly Constructs a Handle

Flags properties whose default value directly constructs a handle instance (`handle()`, `onCleanup(...)`, `containers.Map(...)`, `timer()`, a constructor of the file's own handle class, and similar). The default is evaluated once when the class is loaded, so every instance shares the same object data.

### Why this matters

Shared handle defaults cause surprising aliasing: mutating the property on one instance changes it for every instance created later.

### Examples

#### Incorrect

```matlab
classdef Foo < handle
    properties
        Cleanup = onCleanup(@cleanup)
    end
end
```

#### Correct

```matlab
classdef Foo < handle
    properties
        Cleanup
    end
    methods
        function obj = Foo()
            obj.Cleanup = onCleanup(@cleanup);
        end
    end
end
```

If the sharing is intentional, declare the property `Constant`.

## MCHDT - Property Default Resolves to a Handle

Flags properties whose default value is an identifier or expression that the type environment proves to be a handle-typed value. This fires when the default names a handle value instead of constructing one inline; the sharing concern is the same as MCHDP.

### Why this matters

A property whose default is a handle value is shared by all instances, which is usually unintended.

### Examples

#### Incorrect

```matlab
classdef Foo < handle
    properties
        Cleanup = cleanupObj   % cleanupObj is a handle
    end
end
```

#### Correct

```matlab
classdef Foo < handle
    properties
        Cleanup
    end
    methods
        function obj = Foo()
            obj.Cleanup = onCleanup(@cleanup);
        end
    end
end
```

Heuristic note: the check only fires when the type environment can prove the identifier is a handle. External handle classes that are neither in the built-in list nor the file's own class are not resolved.

## SHVAU - Confusing Shared-Variable Usage

Flags a name that is used inside a nested function and also assigned in the enclosing function *after* the nested function definition. MATLAB cannot tell whether the nested function's use refers to the shared variable or to a separate local, producing confusing behavior.

### Why this matters

Assigning a variable in the parent after a nested function definition makes the nested function's reference ambiguous. Initialize the variable before the nested function to make it a shared variable, or rename one of the two.

### Examples

#### Incorrect

```matlab
function outer()
    y = 1;
    function inner()
        disp(x);   % ambiguous: shared or local?
    end
    x = 2;         % assigned after the nested function
end
```

#### Correct

```matlab
function outer()
    x = 2;         % assigned before the nested function
    function inner()
        disp(x);   % clearly the shared variable
    end
end
```

## GTARG - Function Called with Too Many Arguments

Flags `function_call` nodes that pass more arguments than the callee accepts. The callee arity comes from a same-file function definition first, then from a built-in table (`data/arity.toml`) covering common fixed-arity MATLAB functions.

### Why this matters

Passing extra arguments is usually a mistake and often indicates the wrong function was called or arguments were reordered.

### Examples

#### Incorrect

```matlab
sin(1, 2);
myfunc(1, 2, 3);   % myfunc(a, b) takes two inputs
```

#### Correct

```matlab
sin(1);
myfunc(1, 2);
```

Calls to functions that are neither defined in the same file nor in the built-in table are skipped (cross-file resolution is not implemented).

## LTARG - Function Called with Too Few Arguments

Flags `function_call` nodes that pass fewer arguments than the callee requires. Functions whose last input is `varargin` accept any number of extra arguments and are only checked for too-few calls.

### Why this matters

Missing arguments typically cause runtime errors or silently wrong behavior when the callee uses `nargin`.

### Examples

#### Incorrect

```matlab
disp();
myfunc(1);         % myfunc(a, b) needs two inputs
```

#### Correct

```matlab
disp('hello');
myfunc(1, 2);
```

## Configuration

These checks are part of the Good Practices engine and are disabled individually via the engine's `disabled_checks` list:

```toml title=".mlt.toml"
[lint.rules.GOOD_PRACTICES_ENGINE]
severity = "warn"
disabled_checks = ["CTPCT", "FXSET"]
```

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"warn"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable (e.g., `["CTPCT", "TLEV"]`) |

## Automatic fixes

- `PFRNI` — rewrites an explicit three-part parfor range (`1:2:10`) as a two-part range (`1:10`).
- `COMPNOP` — replaces `call(...) == true` with `call(...)`.
- `COMPNOT` — replaces `call(...) ~= true` or `call(...) == false` with `~call(...)`.

The other checks in this engine do not provide automatic fixes.

## Target node types

- `function_call` — CTPCT, TLEV, UNONC, MIPC1, MNUML, MCCPE, RMFLD, STFLD, RMWRN
- `for_statement` — FXSET, PFRNI, PFEVB, PFGP, PFGV
- `spmd_statement` — SPEVB, SPGV, DSPMDA
- `command` — SIMPT, DUALC
- `comparison_operator` — COMPNOP, COMPNOT, MHERM, STRSZ
- `range` — M3COL
- `class_definition` — ATTF, ATTOF, MCPO, MCSAC, MOBSRV, MDEPIN, MCCPI, MGMD, MCCPE, MTHANS (via `check_file`)
- File-level (via `check_file`) — PFIIN, PFOUS, PFTUSW, PFUIXW, COMFS, SEMFS, BDLGI, BDLOG1, BDLOG2, BDSCA, BDSCI, MCHDP, MCHDT, SHVAU, GTARG, LTARG

## Related rules

- `EVLCS` / `EVLDOT` / `EVLEQ` / `EVLSYS` / `EVLDUAL` / `EVLSEQVAR` — other `eval`-family checks
- `LOAD` — another call whose output should be captured
- `PFEVC` / `SPEVC` (Language Specification) — EVALIN/ASSIGNIN('caller') restrictions in parfor/spmd
- `STCMP` — Use `strcmp`/`strcmpi` instead of `==` for strings
- `SFLD` — Use dynamic field names instead of `setfield`
- `NOANS` — Function result assigned to `ans` implicitly
- `PFSLO` / `PFSLRD` / `PFSLW` (Language Specification) — parfor sliced-variable checks
- `STCI` — Use `strcmpi` for case-insensitive comparison (related to `COMPNOP`/`COMPNOT`)
