---
icon: lucide/list-checks
---

# Rules Reference

## Overview

mlt implements lint rules matching MATLAB's Code Analyzer checks for feature parity. Rules use the same check IDs as MATLAB (e.g., `NOSEMI`, `AGROW`, `naming.class.casing`) and are organized into categories.

## Rule Table

| Rule ID | Category | Description | Default Severity | Auto-fix |
| ------- | -------- | ----------- | ---------------- | -------- |
| [NOSEMI](rules/nosemi.md) | Formatting | Statement without trailing semicolon may produce unintended console output | Info | Yes |
| [MFAMB](rules/readability.md) | Readability | Cannot determine whether name is a variable or function; assumes function | Info | No |
| [FLUDLR](rules/readability.md) | Readability | Nested `flipud(fliplr(x))`/`fliplr(flipud(x))` should use `rot90(x, 2)` | Info | Yes |
| [STLOW](rules/readability.md) | Readability | Unnecessary UPPER/LOWER call in a comparison | Info | Yes |
| [COMNL](rules/readability.md) | Readability | Newline following comma acts as a row separator in a matrix; suggest semicolon or ellipsis | Info | Yes |
| [FVINR](rules/readability.md) | Readability | For readability, add Input attribute to the input arguments block | Info | Yes |
| [NOFIL](rules/incomplete-analysis.md) | Incomplete Analysis | File not found | Error | No |
| [RDERR](rules/incomplete-analysis.md) | Incomplete Analysis | Unable to read file | Error | No |
| [QUIT](rules/incomplete-analysis.md) | Incomplete Analysis | Code analysis did not complete; the analyzer encountered an internal error | Error | No |
| [NOPAR2](rules/syntax-errors.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at `(` on a line | Error | No |
| [EOLPAR](rules/syntax-errors.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of line | Error | No |
| [ENDPAR](rules/syntax-errors.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of file | Error | No |
| [UNSET](rules/syntax-errors.md) | Syntax Errors | Invalid use of operator on the left side of an assignment | Error | No |
| [LHROW](rules/syntax-errors.md) | Syntax Errors | The left side of an assignment cannot have multiple rows (';') | Error | No |
| [RESWD](rules/syntax-errors.md) | Syntax Errors | Invalid use of a reserved word | Error | No |
| [SYNEND](rules/syntax-errors.md) | Syntax Errors | Invalid use for END operator | Error | No |
| [MCPLD](rules/syntax-errors.md) | Syntax Errors | Invalid property syntax | Error | No |
| [BADNOT](rules/syntax-errors.md) | Syntax Errors | Using ~ to ignore a value is not permitted in this context | Error | No |
| [BADNOTLHS](rules/syntax-errors.md) | Syntax Errors | Invalid use of logical not operator (~) on left side of an assignment | Error | No |
| [BADCT](rules/syntax-errors.md) | Syntax Errors | Unicode explicit directional formatting characters are not supported | Error | No |
| [ENDCT2](rules/syntax-errors.md) | Syntax Errors | An END might be missing after a block-opening keyword | Error | No |
| [ENDCT3](rules/syntax-errors.md) | Syntax Errors | An END might be missing before a block-opening keyword | Error | No |
| [ENDCT4](rules/syntax-errors.md) | Syntax Errors | A METHODS block or END might be missing before a function definition | Error | No |
| [STRIN](rules/syntax-errors.md) | Syntax Errors | A quoted character vector is unterminated | Error | No |
| [DOUQT](rules/syntax-errors.md) | Syntax Errors | A double quoted string is unterminated | Error | No |
| [INBLK](rules/syntax-errors.md) | Syntax Errors | A block comment is unterminated at the end of the file | Error | No |
| [BADFP](rules/syntax-errors.md) | Syntax Errors | Invalid floating-point constant | Error | No |
| [BADHBH](rules/syntax-errors.md) | Syntax Errors | Invalid digit in hexadecimal literal | Error | No |
| [BADHBB](rules/syntax-errors.md) | Syntax Errors | Invalid digit in binary literal | Error | No |
| [BADHBHT](rules/syntax-errors.md) | Syntax Errors | Hexadecimal literal has too many digits for specified type suffix | Error | No |
| [BADHBBT](rules/syntax-errors.md) | Syntax Errors | Binary literal has too many digits for specified type suffix | Error | No |
| [HEXTOOLONG](rules/syntax-errors.md) | Syntax Errors | Hexadecimal literal has too many digits | Error | No |
| [BINARYTOOLONG](rules/syntax-errors.md) | Syntax Errors | Binary literal has too many digits | Error | No |
| [SBTMP](rules/syntax-errors.md) | Syntax Errors | Invalid array indexing or function call; chaining outputs after parenthesis is not supported | Error | No |
| [FVSYN](rules/syntax-errors.md) | Syntax Errors | Invalid function argument syntax | Error | No |
| [FVACI](rules/syntax-errors.md) | Syntax Errors | Use of name-value arguments in cell indexing is not supported | Error | No |
| [FVACS](rules/syntax-errors.md) | Syntax Errors | Using a character vector or string as a name in name=value syntax is not supported | Error | No |
| [FVAMI](rules/syntax-errors.md) | Syntax Errors | Name in name-value argument syntax must be a valid MATLAB identifier | Error | No |
| [VTPOD](rules/syntax-errors.md) | Syntax Errors | Specify validation in the following order: size, then class, then functions | Error | No |
| [FVAPN](rules/language-spec.md) | Language Specification | Move name-value arguments that use the name=value syntax to the end of the argument list | Error | No |
| [FVATF](rules/language-spec.md) | Language Specification | Attribute values in arguments blocks must be logical constants | Error | No |
| [FVBTN](rules/language-spec.md) | Language Specification | Use of this function is not supported in arguments blocks | Error | No |
| [FVDAN](rules/language-spec.md) | Language Specification | Same name as both a name-value argument structure and a positional argument | Error | No |
| [FVDAP](rules/language-spec.md) | Language Specification | Positional argument can only be declared once | Error | No |
| [FVDNF](rules/language-spec.md) | Language Specification | Name-value argument can only be declared once | Error | No |
| [FVDREP](rules/language-spec.md) | Language Specification | Multiple Repeating arguments blocks are not supported | Error | No |
| [FVIDV](rules/language-spec.md) | Language Specification | Specifying validation or default value for ignored arguments is not supported | Error | No |
| [FVIOA](rules/language-spec.md) | Language Specification | Both 'Input' and 'Output' attributes on the same arguments block is not supported | Error | No |
| [FVMCL](rules/language-spec.md) | Language Specification | Multiple name-value structures using .? syntax and a class name is not supported | Error | No |
| [FVNDE](rules/language-spec.md) | Language Specification | Default values for class-name name-value arguments are illegal | Error | No |
| [FVNIV](rules/language-spec.md) | Language Specification | Variable is not an input to the function and cannot be used in an arguments block | Error | No |
| [FVNREP](rules/language-spec.md) | Language Specification | Name-value arguments are not supported in a Repeating arguments block | Error | No |
| [FVNSC](rules/language-spec.md) | Language Specification | Use of nested functions is not supported in arguments blocks | Error | No |
| [FVNVL](rules/language-spec.md) | Language Specification | Validation for class-name name-value arguments is illegal | Error | No |
| [FVOBI](rules/language-spec.md) | Language Specification | Declare all input argument blocks before all output arguments blocks | Error | No |
| [FVOCON](rules/language-spec.md) | Language Specification | For output arguments, validation functions must only use the argument being validated or literals | Error | No |
| [FVOND](rules/language-spec.md) | Language Specification | Use of name-value arguments in default values is not supported | Error | No |
| [FVONV](rules/language-spec.md) | Language Specification | Use of name-value arguments without dotted name in the validation is not supported | Error | No |
| [FVOOD](rules/language-spec.md) | Language Specification | Specifying a default value for an output argument is not supported | Error | No |
| [FVOOI](rules/language-spec.md) | Language Specification | Use of ignored arguments in output arguments block is not supported | Error | No |
| [FVOON](rules/language-spec.md) | Language Specification | Using name-value argument as output argument is not supported | Error | No |
| [FVORDI](rules/language-spec.md) | Language Specification | Ignored input arguments are not allowed after a Repeating arguments block or name-value arguments | Error | No |
| [FVORDN](rules/language-spec.md) | Language Specification | Positional arguments must be defined before name-value arguments | Error | No |
| [FVORDO](rules/language-spec.md) | Language Specification | Repeating output arguments must be defined after required output arguments | Error | No |
| [FVORDP](rules/language-spec.md) | Language Specification | Positional arguments must be defined in the following order: required, optional, and repeating | Error | No |
| [FVORM](rules/language-spec.md) | Language Specification | Declaring multiple repeating output arguments is not supported | Error | No |
| [FVOVREP](rules/language-spec.md) | Language Specification | Output argument varargout can only be used inside a Repeating output arguments block | Error | No |
| [FVREPD](rules/language-spec.md) | Language Specification | Default values are not supported in a Repeating arguments block | Error | No |
| [FVREPO](rules/language-spec.md) | Language Specification | Repeating input arguments block containing varargin must not have other arguments | Error | No |
| [FVSOR](rules/language-spec.md) | Language Specification | Input arguments block declarations and the function line must contain the same input arguments in the same order | Error | No |
| [FVSORO](rules/language-spec.md) | Language Specification | Output arguments block declarations and the function line must contain the same output arguments in the same order | Error | No |
| [FVUBD](rules/language-spec.md) | Language Specification | Argument is referenced before it is declared in the arguments block | Error | No |
| [FVVCON](rules/language-spec.md) | Language Specification | For input arguments, validation functions must only use previously declared positional arguments, the argument being validated, or literals | Error | No |
| [FVVIN](rules/language-spec.md) | Language Specification | Validation function must use the argument as an input | Error | No |
| [FVVREP](rules/language-spec.md) | Language Specification | varargin can only be used inside repeating input arguments block | Error | No |
| [TINVALDIM](rules/language-spec.md) | Language Specification | Each dimension must be a nonnegative integer number or a colon | Error | No |
| [TTOOFEWDIMS](rules/language-spec.md) | Language Specification | Specify at least two dimensions for size | Error | No |
| [CTOINE](rules/language-spec.md) | Language Specification | Use of constructed object as input to constructor is not supported | Error | No |
| [CTORO](rules/language-spec.md) | Language Specification | Class constructors must be declared with at least one output argument | Error | No |
| [ERTXT](rules/language-spec.md) | Language Specification | Specify an error message with the message identifier | Error | No |
| [MHERIT](rules/language-spec.md) | Language Specification | Deriving from a built-in MATLAB class is not supported | Error | No |
| [NCHKOS](rules/language-spec.md) | Language Specification | NARGINCHK does not return any values | Error | No |
| [SPBFN](rules/language-spec.md) | Language Specification | Use of a workspace-access function inside an rules/S.md block | Error | No |
| [SPBRK](rules/language-spec.md) | Language Specification | break/continue not fully contained in an rules/S.md block (subsumed by SPRET) | Error | No |
| [SPDEC](rules/language-spec.md) | Language Specification | rules/S.md worker bounds must be nonnegative integers | Error | No |
| [SPDEC3](rules/language-spec.md) | Language Specification | rules/S.md can only specify lower and upper worker bounds | Error | No |
| [SPEVC](rules/language-spec.md) | Language Specification | EVALIN/ASSIGNIN('caller') invalid inside an rules/S.md block | Error | No |
| [SPLD](rules/language-spec.md) | Language Specification | Assign the output of LOAD to a variable in rules/S.md blocks | Error | No |
| [SPNF](rules/language-spec.md) | Language Specification | Nested function call inside an rules/S.md block | Error | No |
| [SPSV](rules/language-spec.md) | Language Specification | SAVE requires the '-fromstruct' option inside an rules/S.md block | Error | No |
| [SPWHOS](rules/language-spec.md) | Language Specification | who/whos without '-file' is invalid inside an rules/S.md block | Error | No |
| [USESWNS](rules/language-spec.md) | Language Specification | Variable must be explicitly defined before first use | Error | No |
| [WTXT](rules/language-spec.md) | Language Specification | Specify a warning message with the message identifier | Error | No |
| [MABSEAC](rules/language-spec.md) | Language Specification | Instance properties and methods are illegal in classes that are both Sealed and Abstract | Error | No |
| [MABSEAM](rules/language-spec.md) | Language Specification | A method cannot be both Abstract and Sealed | Error | No |
| [MCAPP](rules/language-spec.md) | Language Specification | Private property cannot be Abstract | Error | No |
| [MCCBS](rules/language-spec.md) | Language Specification | A superclass constructor is being called, but the name is not a declared superclass name | Error | No |
| [MCCBU](rules/language-spec.md) | Language Specification | This superclass constructor is called after a use of the constructed object | Error | No |
| [MCCMC](rules/language-spec.md) | Language Specification | Constructor for superclass can only be called once | Error | No |
| [MCCSOP](rules/language-spec.md) | Language Specification | Unable to modify Constant property | Error | No |
| [MCGSA](rules/language-spec.md) | Language Specification | Method tries to set or get an abstract property | Error | No |
| [MCMIO](rules/language-spec.md) | Language Specification | Method has too many inputs or outputs | Error | No |
| [MCMSP](rules/language-spec.md) | Language Specification | Private method cannot be Abstract | Error | No |
| [MCMTP](rules/language-spec.md) | Language Specification | TestParameterDefinition methods must be Static | Error | No |
| [MCPIN](rules/language-spec.md) | Language Specification | Unable to initialize class property to an instance of the class itself | Error | No |
| [MCPSG](rules/language-spec.md) | Language Specification | Set or get method must be fully defined in the class definition file | Error | No |
| [MCSCC](rules/language-spec.md) | Language Specification | To call the superclass constructor, the subclass constructor name must match the subclass name | Error | No |
| [MCSCF](rules/language-spec.md) | Language Specification | A superclass constructor must be assigned to the first constructor output argument | Error | No |
| [MCSCM](rules/language-spec.md) | Language Specification | To call a superclass method, the method name must match the subclass method name | Error | No |
| [MCSCN](rules/language-spec.md) | Language Specification | Method tries to set a constant property | Error | No |
| [MCSCO](rules/language-spec.md) | Language Specification | A superclass constructor must be called using the first constructor output argument | Error | No |
| [MCSCT](rules/language-spec.md) | Language Specification | Superclass constructor call must not be conditionalized or be part of another expression | Error | No |
| [MCSMO](rules/language-spec.md) | Language Specification | Returning multiple outputs from a superclass object initialization is not supported | Error | No |
| [MCSWA](rules/language-spec.md) | Language Specification | A sealed class cannot specify allowed subclasses | Error | No |
| [MTAGS3](rules/language-spec.md) | Language Specification | Cannot use the Access attribute when using the SetAccess or GetAccess attribute | Error | No |
| [MTMAT](rules/language-spec.md) | Language Specification | Attribute can only be set once | Error | No |
| [MWKCL](rules/language-spec.md) | Language Specification | A WeakHandle property must restrict its type using a class validation | Error | No |
| [MWKCT](rules/language-spec.md) | Language Specification | Specifying both WeakHandle and Constant attributes on the same property is not supported | Error | No |
| [MWKREF](rules/language-spec.md) | Language Specification | Specifying both WeakHandle and Dependent attributes is invalid | Error | No |
| [FPFORP](rules/language-spec.md) | Language Specification | 'fprintf' is writing to a file opened with read permission only | Error | No |
| [FWFORP](rules/language-spec.md) | Language Specification | 'fwrite' is writing to a file opened with read permission only | Error | No |
| [PFANON](rules/language-spec.md) | Language Specification | Sliced output variable used in an anonymous function is not supported in parfor loops | Error | No |
| [PFANSLP](rules/language-spec.md) | Language Specification | 'ans' is not supported as a parfor loop variable | Error | No |
| [PFANSNS](rules/language-spec.md) | Language Specification | 'ans' is not supported as a for loop variable in parfor loops | Error | No |
| [PFCEL](rules/language-spec.md) | Language Specification | The function does not support cell arrays (argument N) | Error | No |
| [PFCTXT](rules/language-spec.md) | Language Specification | Sliced variable indexed with a nested for loop variable must be inside the for loop that defines its range | Error | No |
| [PFEVC](rules/language-spec.md) | Language Specification | EVALIN('caller') and ASSIGNIN('caller') are invalid inside a PARFOR loop | Error | No |
| [PFFRNG](rules/language-spec.md) | Language Specification | When indexing a sliced variable with a nested for loop variable, the range must be a row vector of positive constant numbers | Error | No |
| [PFFSUB](rules/language-spec.md) | Language Specification | Indexing a nested for loop variable is not supported in parfor loops | Error | No |
| [PFINCR](rules/language-spec.md) | Language Specification | Using different reduction functions with the same reduction variable is not supported in parfor loops | Error | No |
| [PFINPT](rules/language-spec.md) | Language Specification | 'inputname' is not supported in parfor loops | Error | No |
| [PFLD](rules/language-spec.md) | Language Specification | 'load' must assign to an output variable in parfor loops | Error | No |
| [PFMLTI](rules/language-spec.md) | Language Specification | The nested for loop variable must not be assigned other than by its for statement | Error | No |
| [PFNACK](rules/language-spec.md) | Language Specification | 'narginchk' and 'nargoutchk' cannot be used in parfor loops | Error | No |
| [PFNAIO](rules/language-spec.md) | Language Specification | 'nargin' and 'nargout' require a function argument in parfor loops | Error | No |
| [PFNAR](rules/language-spec.md) | Language Specification | Subtracting reduction variable from expressions is not supported in parfor loops | Error | No |
| [PFRFH](rules/language-spec.md) | Language Specification | The PARFOR reduction function must be a function name or a broadcast variable | Error | No |
| [PFRNG](rules/language-spec.md) | Language Specification | The range of a PARFOR statement must be increasing consecutive integers | Error | No |
| [PFSLO](rules/language-spec.md) | Language Specification | Variable is indexed using the parfor loop variable, but it is not a valid sliced output variable | Error | No |
| [PFSLRD](rules/language-spec.md) | Language Specification | Invalid combination of sliced indexing and non-indexed reads of a sliced output variable | Error | No |
| [PFSLW](rules/language-spec.md) | Language Specification | Multiple sliced accesses must all use the same list of subscripts | Error | No |
| [PFSV](rules/language-spec.md) | Language Specification | SAVE cannot be called in a PARFOR loop without the '-fromstruct' option | Error | No |
| [PFUNK](rules/language-spec.md) | Language Specification | The PARFOR loop cannot run due to the way variable VAR_NAME is used | Error | No |
| [PFUTMP](rules/language-spec.md) | Language Specification | Temporary variable VAR_NAME must be set inside the parfor loop before it is used | Error | No |
| [PFUTVR](rules/language-spec.md) | Language Specification | Variable VAR_NAME may have been intended as a reduction variable, but is an uninitialized temporary | Error | No |
| [PFVARS](rules/language-spec.md) | Language Specification | Parfor loop contains too many variables | Error | No |
| [PFVSUB](rules/language-spec.md) | Language Specification | Indexing parfor loop variables is not supported in parfor loops | Error | No |
| [COMFS](rules/good-practices.md) | Good Practices | This comma makes the file a script; all functions are local functions | Warning | No |
| [DUALC](rules/good-practices.md) | Good Practices | Command might be prematurely ended by comma | Warning | No |
| [RMFLD](rules/good-practices.md) | Good Practices | RMFIELD output must be assigned back to the structure | Warning | No |
| [RMWRN](rules/good-practices.md) | Good Practices | Warning with a removed tag has no effect | Warning | No |
| [SEMFS](rules/good-practices.md) | Good Practices | This semicolon makes the file a script; all functions are local functions | Warning | No |
| [STFLD](rules/good-practices.md) | Good Practices | SETFIELD output must be assigned back to the structure | Warning | No |
| [STRSZ](rules/good-practices.md) | Good Practices | Use STRCMP to compare character vectors that can have different sizes | Warning | No |
| [COMPNOP](rules/good-practices.md) | Good Practices | Comparison with `true` simplifies to the function call itself | Warning | Yes |
| [COMPNOT](rules/good-practices.md) | Good Practices | Comparison with `~= true` or `== false` simplifies to `~call(...)` | Warning | Yes |
| [M3COL](rules/good-practices.md) | Good Practices | Three colons (`a:b:c:d`) in an expression is probably unintended | Warning | No |
| [CTPCT](rules/good-practices.md) | Good Practices | The format might not agree with the argument count | Warning | No |
| [FXSET](rules/good-practices.md) | Good Practices | Loop index variable is changed inside of a FOR loop | Warning | No |
| [SIMPT](rules/good-practices.md) | Good Practices | Import statement does not run first in a function | Warning | No |
| [TLEV](rules/good-practices.md) | Good Practices | Dynamic-code function used as a sub-expression, not a top-level statement | Warning | No |
| [UNONC](rules/good-practices.md) | Good Practices | Assign the onCleanup output argument to a variable, not `~` | Warning | No |
| [MIPC1](rules/good-practices.md) | Good Practices | `computer('arch')` returns a platform-specific value | Warning | No |
| [ATTF](rules/good-practices.md) | Good Practices | Unable to determine if the expression assigned to the `Abstract` attribute evaluates to true or false | Warning | No |
| [ATTOF](rules/good-practices.md) | Good Practices | Setting the class attribute `Abstract` to false is not recommended | Info | No |
| [MCPO](rules/good-practices.md) | Good Practices | `SetObservable`/`GetObservable`/`AbortSet` property has no effect in a value class | Warning | No |
| [MCSAC](rules/good-practices.md) | Good Practices | `SetAccess` cannot be set on Constant properties | Warning | No |
| [MOBSRV](rules/good-practices.md) | Good Practices | `SetObservable`/`GetObservable` on a Constant property has no effect | Info | No |
| rules/.mdEPIN](rules/good-practices.md) | Good Practices | Default values should not be assigned to dependent properties | Warning | No |
| [MCCPI](rules/good-practices.md) | Good Practices | Initialize the Constant property or make it an Abstract Constant property | Warning | No |
| [rules/M.md](rules/good-practices.md) | Good Practices | `get` method should be implemented for each dependent property without private `GetAccess` | Warning | No |
| [MCCPE](rules/good-practices.md) | Good Practices | Attempting to call a property or event as a function | Warning | No |
| [MTHANS](rules/good-practices.md) | Good Practices | Using `ANS` as a method name is not recommended | Info | No |
| [MHERM](rules/good-practices.md) | Good Practices | Parenthesize the multiplication of a variable and its transpose to ensure the result is Hermitian | Info | No |
| [MNUML](rules/good-practices.md) | Good Practices | To create a square matrix, use `VAR_NAME(numel(...), numel(...))` | Warning | No |
| [PFEVB](rules/good-practices.md) | Good Practices | Using EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop refers to the worker machines' base workspaces | Warning | No |
| [PFGP](rules/good-practices.md) | Good Practices | Avoid assigning to GLOBAL or PERSISTENT variable inside a PARFOR loop | Warning | No |
| [PFGV](rules/good-practices.md) | Good Practices | Avoid using GLOBAL variable in a PARFOR loop | Warning | No |
| [PFIIN](rules/good-practices.md) | Good Practices | The input variable should be initialized before the PARFOR loop | Warning | No |
| [PFOUS](rules/good-practices.md) | Good Practices | The output variable might not be used after the PARFOR loop | Warning | No |
| [PFRNI](rules/good-practices.md) | Good Practices | Do not specify the increment explicitly; the parfor loop can only use an increment of one | Warning | Yes |
| [PFTUSW](rules/good-practices.md) | Good Practices | The temporary variable might be used after the PARFOR loop | Warning | No |
| [PFUIXW](rules/good-practices.md) | Good Practices | The index variable might be used after the PARFOR loop | Warning | No |
| [SPEVB](rules/good-practices.md) | Good Practices | Using EVALIN('base') or ASSIGNIN('base') inside an rules/S.md block refers to the worker machines' base workspaces | Warning | No |
| [SPGV](rules/good-practices.md) | Good Practices | Using the GLOBAL or PERSISTENT variable in an rules/S.md block might fail because it is accessed on a worker machine | Warning | No |
| [rules/DS.mdA](rules/good-practices.md) | Good Practices | Distributed array must be created outside of an rules/S.md block | Warning | No |
| [ADMTHDINV](rules/good-practices.md) | Good Practices | Use `VAR_NAME(app, ...)` to call this function | Warning | No |
| [ADPROP](rules/good-practices.md) | Good Practices | Use `app.VAR_NAME` to refer to this property | Warning | No |
| [ADPROPLC](rules/good-practices.md) | Good Practices | Use `app.VAR_NAME` to reference a property of app | Warning | No |
| [CTOINW](rules/good-practices.md) | Good Practices | Use of constructed object as input to constructor is not necessary | Warning | No |
| [FXUP](rules/good-practices.md) | Good Practices | Outer loop index is set inside a nested function | Warning | No |
| [MCCSPS](rules/good-practices.md) | Good Practices | Constant property is not modified; `VAR_NAME.VAR_NAME` creates a struct | Warning | No |
| [MCNPN](rules/good-practices.md) | Good Practices | Referenced but is not a property, method, or event name defined in this class | Warning | No |
| [MCNPR](rules/good-practices.md) | Good Practices | Not a property, but is the target of an assignment | Warning | No |
| [MCSNOV](rules/good-practices.md) | Good Practices | Set function in value class must return the modified object | Warning | No |
| [MCSOH](rules/good-practices.md) | Good Practices | Set function in handle class does not need to return the modified object | Warning | No |
| [MCSUP](rules/good-practices.md) | Good Practices | The set method for a property should not access another property | Warning | No |
| [MCVM](rules/good-practices.md) | Good Practices | Value class method that modifies the object must return the modified object | Warning | No |
| [PFRIN](rules/good-practices.md) | Good Practices | The reduction variable might not be set before the PARFOR loop | Warning | No |
| [PFRUS](rules/good-practices.md) | Good Practices | The reduction variable might not be used after the PARFOR loop | Warning | No |
| [SUBSINDEX](rules/good-practices.md) | Good Practices | Do not overload `subsindex` for fundamental data types | Warning | No |
| [VTFIN](rules/good-practices.md) | Good Practices | The validated value should be the first input argument to the validator function | Warning | No |
| [BDLGI](rules/good-practices.md) | Good Practices | Variable might be set by a nonlogical operator | Warning | No |
| [BDLOG1](rules/good-practices.md) | Good Practices | A scalar logical value is expected in the conditional expression; use `any` or `all` to reduce the array to a logical scalar | Warning | No |
| [BDLOG2](rules/good-practices.md) | Good Practices | A scalar logical value is expected in the conditional expression; use `any` or `all`, or compare the scalar value to 0 | Warning | No |
| [BDSCA](rules/good-practices.md) | Good Practices | Unexpected use of `&&`/`||` in a scalar context | Warning | No |
| [BDSCI](rules/good-practices.md) | Good Practices | Variable might be set by a nonscalar operator | Warning | No |
| [MCHDP](rules/good-practices.md) | Good Practices | Property default value that is a handle is shared by all instances | Warning | No |
| [MCHDT](rules/good-practices.md) | Good Practices | Property default value that resolves to a handle is shared by all instances | Warning | No |
| [SHVAU](rules/good-practices.md) | Good Practices | Confusing usage of a name assigned after a nested function definition | Warning | No |
| [GTARG](rules/good-practices.md) | Good Practices | Function might be called with too many arguments | Warning | No |
| [LTARG](rules/good-practices.md) | Good Practices | Function might be called with too few arguments | Warning | No |
| [SOINITPROP](rules/system-objects.md) | System Objects | Initialize DiscreteState property within a `resetImpl` method | Warning | No |
| [SOTUNPROP1](rules/system-objects.md) | System Objects | Logical attribute not supported for tunable properties on MATLAB System blocks | Warning | No |
| [SOTUNPROP3](rules/system-objects.md) | System Objects | Tunable properties on System blocks must be numeric; char property is made Nontunable | Warning | No |
| [SOTUNPROP4](rules/system-objects.md) | System Objects | Tunable properties on System blocks must be numeric; string property is made Nontunable | Warning | No |
| [ATAS](rules/language-spec.md) | Language Specification | The attribute value is unexpected. Use a single meta-class object or a cell array of meta-class objects | Error | No |
| [ATLAB](rules/language-spec.md) | Language Specification | Attribute 'Input' and 'Output' must not be assigned a value or negated | Error | No |
| [ATNAS](rules/language-spec.md) | Language Specification | Set attribute to a single meta-class object or a cell array of meta-class objects | Error | No |
| [ATNPI](rules/language-spec.md) | Language Specification | Set attribute to 'public', 'private', 'protected', 'immutable', or a cell array of meta-classes instead | Error | No |
| [ATNPP](rules/language-spec.md) | Language Specification | Set attribute to 'public', 'private', 'protected', or a cell array of meta-classes instead | Error | No |
| [ATPPI](rules/language-spec.md) | Language Specification | The attribute value is unexpected. Use 'public', 'private', 'protected', 'immutable', or a cell array of meta-classes instead | Error | No |
| [ATPPP](rules/language-spec.md) | Language Specification | The attribute value is unexpected. Use 'public', 'private', 'protected', or a cell array of meta-classes instead | Error | No |
| [ATUNK](rules/language-spec.md) | Language Specification | Unknown attribute name | Error | No |
| [ATVIZE](rules/language-spec.md) | Language Specification | The 'Visible' attribute is invalid for classes and events. Use the '~Hidden' attribute instead | Error | No |
| [CLSAT](rules/language-spec.md) | Language Specification | Specify class attributes before the name of the class | Error | No |
| [CLSUNK](rules/language-spec.md) | Language Specification | This class, or one of its superclasses, could not be found on MATLAB's path | Error | No |
| [NOPRV](rules/language-spec.md) | Language Specification | A class definition cannot be inside a private directory | Error | No |
| [PFANSRE](rules/language-spec.md) | Language Specification | 'ans' is not supported as a reduction variable in parfor loops | Error | No |
| [PFANSSL](rules/language-spec.md) | Language Specification | 'ans' is not supported as a sliced variable in parfor loops | Error | No |
| [PFDF](rules/language-spec.md) | Language Specification | FOR with DRANGE (old PARFOR) becomes a conventional FOR when used inside a PARFOR loop | Error | No |
| [PFPIE](rules/language-spec.md) | Language Specification | Valid indices for the variable are restricted in PARFOR loops | Error | No |
| [PFSAME](rules/language-spec.md) | Language Specification | In a PARFOR loop, the variable is indexed in different ways, potentially causing dependencies between iterations | Error | No |
| [PFTIN](rules/language-spec.md) | Language Specification | The temporary variable must be set inside the PARFOR loop before it is used | Error | No |
| [VTPCON](rules/language-spec.md) | Language Specification | For properties, validation functions must only use the property being validated or literals | Error | No |
| [VTPEAL](rules/language-spec.md) | Language Specification | Specify at least one input argument for validator | Error | No |
| [VTPIN](rules/language-spec.md) | Language Specification | Validation function must use the property as an input | Error | No |

## Data-Driven Check IDs

These tables are generated at build time from the TOML data files by
[markdown-exec](https://zensical.org/docs/setup/extensions/markdown-exec/); the
source generator lives in `tools/gen_rules_docs.py`. Rebuild the docs (`zensical
build`) to refresh them whenever `data/compatibility.toml` or
`data/suggested_improvements.toml` changes.

```python exec="on"
import sys
sys.path.insert(0, "tools")
from gen_rules_docs import render_data_driven_tables
print(render_data_driven_tables())
```

## Rule Categories

mlt organizes rules into the same categories as MATLAB's Code Analyzer:

| Category | Config Key | Description |
| -------- | ---------- | ----------- |
| Incomplete Analysis | `incomplete-analysis` | Internal linter limits and analysis failures |
| Syntax Errors | `syntax-errors` | Parser-level syntax validation |
| Language Specification | `language-specification` | Language constraint violations |
| Bugs | `bugs` | Likely bugs and logic errors |
| Custom Checks | `custom-checks` | Configurable code complexity/style metrics |
| Naming | `naming` | Naming convention enforcement |
| Compatibility | `compatibility` | Deprecated/removed functions and APIs |
| Forward Compatibility | `forward-compatibility` | Forward compatibility issues |
| Good Practices | `good-practices` | Common best practices |
| Unset Variables | `unset-variables` | Variables that may not be defined before use |
| Unused Constructions | `unused-constructions` | Dead code and unused constructions |
| Suggested Improvements | `suggested-improvements` | Suggestions for improved code patterns |
| Readability | `readability` | Readability improvements |
| Formatting | `formatting` | Code formatting suggestions |
| Performance | `performance` | Performance improvement suggestions |
| Code Generation | `code-generation` | MATLAB Coder constraints |
| Fixed-Point | `fixed-point` | Fixed-point toolbox specific |
| Deployment | `deployment` | MATLAB Compiler deployment constraints |
| System Objects | `system-objects` | System object validation |
| Unsupported | `unsupported` | Unsupported features |
| Behavior Changes | `behavior-changes` | Behavior changes between MATLAB versions |
| Configuration Issues | `configuration-issues` | Configuration file validation |

## Severity Levels

Rules are categorized into three severity levels:

### Error

Critical issues that likely indicate bugs or broken code:

- Code that will fail at runtime
- Language specification violations
- Syntax errors

### Warning

Issues that affect code quality:

- Common bug patterns
- Good practice violations
- Deprecated function usage

### Info

Low-priority suggestions:

- Formatting preferences
- Performance improvement hints
- Readability suggestions

### Configuring Severity

Override default severities in `.mlt.toml`:

```toml
[lint.rules]
NOSEMI = "error"    # Upgrade from info to error
```

Or in a full table:

```toml
[lint.rules.NOSEMI]
severity = "error"
ignore_functions = ["disp", "fprintf"]
```

### Category-Level Configuration

Enable, disable, or override severity for entire categories:

```toml
[lint.categories]
performance = "off"           # Disable all performance rules
compatibility = "warn"        # Override all compatibility rules to warning
formatting = "info"           # Set all formatting rules to info
```

Per-rule configuration always takes precedence over category-level settings.

See [Configuration](configuration.md) for full details.

## Enabling and Disabling Rules

All rules are enabled by default. Disable individual rules:

```toml
[lint.rules]
NOSEMI = "off"
```

Or disable entire categories:

```toml
[lint.categories]
behavior-changes = "off"
```

## Auto-fix Support

Rules marked with "Yes" in the Auto-fix column provide automatic fixes. Run mlt with `--fix` to apply them:

```bash
mlt --fix src/**/*.m
```

Fixes are applied atomically per file. Overlapping fixes are detected and the conflicting fix is skipped with a warning.

## Adding New Rules

mlt uses auto-registration via the `inventory` crate. To add a new rule:

1. Create a new module in `crates/mlt_rules/src/` (e.g., `agrow.rs`)
2. Implement the `Rule` trait with a `from_config` factory
3. Add `inventory::submit!(crate::RuleRegistration::new("AGROW", Agrow::from_config));` at the bottom
4. Add documentation in `docs/rules/agrow.md`

No manual edits to `lib.rs` are needed beyond adding the `pub mod` declaration.

See the [repository](https://github.com/watermarkhu/mlt) for the full development guide.
