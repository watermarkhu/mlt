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
| [SPBFN](rules/language-spec.md) | Language Specification | Use of a workspace-access function inside an SPMD block | Error | No |
| [SPBRK](rules/language-spec.md) | Language Specification | break/continue not fully contained in an SPMD block (subsumed by SPRET) | Error | No |
| [SPDEC](rules/language-spec.md) | Language Specification | SPMD worker bounds must be nonnegative integers | Error | No |
| [SPDEC3](rules/language-spec.md) | Language Specification | SPMD can only specify lower and upper worker bounds | Error | No |
| [SPEVC](rules/language-spec.md) | Language Specification | EVALIN/ASSIGNIN('caller') invalid inside an SPMD block | Error | No |
| [SPLD](rules/language-spec.md) | Language Specification | Assign the output of LOAD to a variable in SPMD blocks | Error | No |
| [SPNF](rules/language-spec.md) | Language Specification | Nested function call inside an SPMD block | Error | No |
| [SPSV](rules/language-spec.md) | Language Specification | SAVE requires the '-fromstruct' option inside an SPMD block | Error | No |
| [SPWHOS](rules/language-spec.md) | Language Specification | who/whos without '-file' is invalid inside an SPMD block | Error | No |
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
| [MDEPIN](rules/good-practices.md) | Good Practices | Default values should not be assigned to dependent properties | Warning | No |
| [MCCPI](rules/good-practices.md) | Good Practices | Initialize the Constant property or make it an Abstract Constant property | Warning | No |
| [MGMD](rules/good-practices.md) | Good Practices | `get` method should be implemented for each dependent property without private `GetAccess` | Warning | No |
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
| [SPEVB](rules/good-practices.md) | Good Practices | Using EVALIN('base') or ASSIGNIN('base') inside an SPMD block refers to the worker machines' base workspaces | Warning | No |
| [SPGV](rules/good-practices.md) | Good Practices | Using the GLOBAL or PERSISTENT variable in an SPMD block might fail because it is accessed on a worker machine | Warning | No |
| [DSPMDA](rules/good-practices.md) | Good Practices | Distributed array must be created outside of an SPMD block | Warning | No |
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

These tables are generated from the TOML data files by the TypeScript
generator in `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`)
to refresh them whenever `data/compatibility.toml` or
`data/suggested_improvements.toml` changes.

### Compatibility Considerations (1012 checks)

| Check ID | Message |
| -------- | ------- |
| `DPSD` | 'psd' has been removed. Use 'periodogram' or 'pwelch' instead. |
| `DSPDF` | 'dsp.DigitalFilter' has been removed. Use 'dsp.FIRFilter', 'dsp.IIRFilter', or 'dsp.AllpoleFilter' instead. |
| `DMSSPEC` | 'dspdata.msspectrum' will be removed in a future release. Use 'periodogram' or 'pwelch' instead. |
| `DPSPEC` | 'dspdata.pseudospectrum' will be removed in a future release. Use 'pmusic' or 'peig' instead. |
| `DPPSD` | 'dspdata.psd' will be removed in a future release. Use 'pburg', 'pcov', 'peig', 'pmcov', 'pmtm', 'periodogram', or 'pwelch' instead. |
| `DINLN` | INLINE will be removed in a future release. Use anonymous functions instead. |
| `DFCNCHK` | FCNCHK will be removed in a future release. Use anonymous functions instead. |
| `MSYSTEM` | matlab.system.System has been removed. Use matlab.System instead. |
| `MATPOOL` | 'matlabpool' has been removed. Use 'pool' instead. |
| `OBJMPOOL` | 'MATLABPOOL' has been removed. Use 'PARPOOL' instead. |
| `TREEDISP` | TREEDISP has been removed. Use ClassificationTree or RegressionTree VIEW methods instead. |
| `TREEPRUNE` | TREEPRUNE has been removed. Use ClassificationTree or RegressionTree PRUNE methods instead. |
| `TREETEST` | 'treetest' has been removed. Use ClassificationTree or RegressionTree methods instead. |
| `TREEVAL` | TREEVAL has been removed. Use ClassificationTree or RegressionTree PREDICT methods instead. |
| `TREEFIT` | TREEFIT has been removed. Use fitctree or fitrtree instead. |
| `FISGET` | 'getfis' has been removed. Access FIS properties using dot notation instead. |
| `FISM2M` | 'mf2mf' has been removed. Convert membership functions using dot notation on 'fismf' objects instead. |
| `FISSET` | 'setfis' has been removed. Set FIS properties using dot notation instead. |
| `FISSHW` | 'showfis' has been removed. View FIS properties using dot notation instead. |
| `WLGC` | 'wlanGeneratorConfig' has been removed. Use the name-value pair syntax of 'wlanWaveformGenerator' instead. |
| `SMTHC` | 'smithchart' has been removed. Use 'smithplot' instead. |
| `WLRC` | 'wlanRecoveryConfig' has been removed. Instead, parameterize the function that accepts the 'wlanRecoveryConfig' object by using the name-value pair syntax. |
| `JAVFM` | 'JavaFrame' was undocumented and has been removed. There is no simple replacement for this. |
| `JAVCT` | 'JavaContainer' was undocumented and has been removed. There is no simple replacement for this. |
| `JAVCM` | 'javacomponent' is undocumented and will be removed in a future release. There is no simple replacement for this. |
| `COMMERRATE` | 'commtest.ErrorRate' has been removed. Use 'comm.ErrorRate' or BERTool instead. |
| `TCRESULT` | 'testconsole.Results' has been removed. Use 'comm.ErrorRate' or BERTool instead. |
| `OPGLI` | 'opengl' has been removed. There is no simple replacement for this. |
| `CNNCGD` | 'cnncodegen' with default 'targetlib' as 'cudnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| `COMMSCOPEED` | 'commscope.eyediagram' has been removed. For line plotting, use the eyediagram function. There is no simple replacement for histogram plotting and measurement analysis. |
| `COMMED` | 'comm.EyeDiagram' has been removed. For line plotting, use the eyediagram function. There is no simple replacement for histogram plotting and measurement analysis. |
| `MKRMT` | 'makerefmat' has been removed. With appropriate code changes, construct a raster reference object using 'georefcells', 'georefpostings', 'georasterref', 'maprefcells', 'maprefpostings' or 'maprasterref' instead. |
| `WFMRM` | 'worldFileMatrixToRefmat' has been removed. With appropriate code changes, construct a raster reference object using 'georasterref' or 'maprasterref' instead. |
| `RV2MAT` | 'refvec2mat' has been removed. With appropriate code changes, construct a geographic raster reference object using 'refvecToGeoRasterReference' instead. |
| `RM2VEC` | 'refmat2vec' has been removed. With appropriate code changes, construct a geographic raster reference object using 'refvecToGeoRasterReference' instead. |
| `SIZEM` | 'sizem' has been removed. With appropriate code changes, use 'rastersize' property of a map raster reference object instead. |
| `LIMIM` | 'limitm' has been removed. With appropriate code changes, use 'LatitudeLimits' and 'LongitudeLimits' properties of a geographic raster reference object instead. |
| `MAPBX` | 'mapbbox' has been removed. With appropriate code changes, use 'XWorldLimits' and 'YWorldLimits' properties of a map raster reference object instead. |
| `DBITMAX` | 'bitmax' has been removed. Use 'flintmax' instead. |
| `COLORDEF` | 'colordef' will be removed in a future release. There is no simple replacement for this. |
| `GRAYMON` | 'graymon' will be removed in a future release. There is no simple replacement for this. |
| `WHITEBG` | 'whitebg' will be removed in a future release. There is no simple replacement for this. |
| `PRINTOPT` | 'printopt' will be removed in a future release. There is no simple replacement for this. |
| `HGEXPORT` | 'hgexport' will be removed in a future release. Use 'exportgraphics' or 'copygraphics' instead. |
| `HANK2SYS` | 'hank2sys' will be removed in a future release. There is no simple replacement for this. |
| `HGSAVE` | 'hgsave' will be removed in a future release. Use 'savefig' instead. |
| `HAND2STCT` | 'handle2struct' will be removed in a future release. There is no simple replacement for this. |
| `HILBIIR` | 'hilbiir' will be removed in a future release. There is no simple replacement for this. |
| `MOVIE2` | 'movie2avi' will be removed in a future release. Use 'VideoWriter' instead. |
| `SERIAL` | 'serial' will be removed in a future release. Use 'serialport' instead. |
| `GPIB` | 'gpib' will be removed in a future release. Use 'visadev' instead. |
| `VISA` | 'visa' will be removed in a future release. Use 'visadev' instead. |
| `UDPP` | 'udp' will be removed in a future release. Use 'udpport' instead. |
| `BLUTH` | 'Bluetooth' (Instrument Control) will be removed in a future release. Use 'bluetooth' instead. |
| `IMTOOL` | 'imtool' will be removed in a future release. Use 'imageViewer' instead. |
| `IMJAV` | 'imjava' will be removed in a future release. There is no simple replacement for this. |
| `DPOOL` | 'parpool' legacy syntax will be removed in a future release. Use updated syntax instead. |
| `MUPAD` | 'mupad' will be removed in a future release. Use MATLAB Live Editor instead. |
| `FBUILDER` | 'filterBuilder' will be removed in a future release. Use 'designfilt' instead. |
| `FDATOOL` | 'fdatool' will be removed in a future release. Use 'filterDesigner' instead. |
| `WINTOOL` | 'wintool' will be removed in a future release. Use 'windowDesigner' instead. |
| `FVTOOL` | 'fvtool' will be removed in a future release. Use 'filterAnalyzer' instead. |
| `FISNEW` | 'newfis' will be removed in a future release. Use 'mamfis' or 'sugfis' instead. |
| `FISADV` | 'addvar' will be removed in a future release. Use 'addInput' or 'addOutput' instead. |
| `FISRMV` | 'rmvar' will be removed in a future release. Use 'removeInput' or 'removeOutput' instead. |
| `FISRMF` | 'rmmf' will be removed in a future release. Use 'removeMF' instead. |
| `FISM2S` | 'mfedit' will be removed in a future release. There is no simple replacement for this. |
| `FISPSR` | 'parsrule' will be removed in a future release. Use 'addRule' instead. |
| `NPI2PI` | 'npi2pi' will be removed in a future release. Use 'wrapToPi' instead. |
| `NPI22PI` | 'zero22pi' will be removed in a future release. Use 'wrapTo2Pi' instead. |
| `CLASSREGTREE` | 'classregtree' will be removed in a future release. Use 'fitctree' or 'fitrtree' instead. |
| `SVMCLASSIFY` | 'svmclassify' will be removed in a future release. Use 'predict' on a trained SVM model instead. |
| `SVMTRAIN` | 'svmtrain' will be removed in a future release. Use 'fitcsvm' instead. |
| `PRINCOMP` | 'princomp' will be removed in a future release. Use 'pca' instead. |
| `PROBDIST` | 'ProbDist' will be removed in a future release. Use probability distribution objects instead. |
| `PROBDISTPARAMETRIC` | 'ProbDistParametric' will be removed in a future release. Use probability distribution objects instead. |
| `PROBDISTKERNEL` | 'ProbDistKernel' will be removed in a future release. Use probability distribution objects instead. |
| `PROBDISTUNIVKERNEL` | 'ProbDistUnivKernel' will be removed in a future release. Use probability distribution objects instead. |
| `PROBDISTUNIVPARAM` | 'ProbDistUnivParam' will be removed in a future release. Use probability distribution objects instead. |
| `FITNAIVEBAYES` | 'NaiveBayes.fit' will be removed in a future release. Use 'fitcnb' instead. |
| `CAPABLE` | 'capable' will be removed in a future release. Use 'capability' instead. |
| `EWMAPLOT` | 'ewmaplot' will be removed in a future release. Use 'controlchart' instead. |
| `SCHART` | 'schart' will be removed in a future release. Use 'controlchart' instead. |
| `XBARPLOT` | 'xbarplot' will be removed in a future release. Use 'controlchart' instead. |
| `RANDSD` | 'randseed' will be removed in a future release. Use 'rng' instead. |
| `DRNDINT` | 'randint' will be removed in a future release. Use 'randi' instead. |
| `ISGLOB` | 'isglobal' will be removed in a future release. There is no simple replacement for this. |
| `DGRAPHICSVER` | 'graphicsversion' will be removed in a future release. There is no simple replacement for this. |
| `DNOANI` | 'noanimate' will be removed in a future release. There is no simple replacement for this. |
| `EPSM` | 'epsm' will be removed in a future release. Use 'eps' instead. |
| `DEXIFRD` | 'exifread' will be removed in a future release. Use 'imfinfo' instead. |
| `HDFGD` | 'hdfgd' will be removed in a future release. There is no simple replacement for this. |
| `HDFSD` | 'hdfsd' will be removed in a future release. There is no simple replacement for this. |
| `HDFSW` | 'hdfsw' will be removed in a future release. There is no simple replacement for this. |
| `HDFTL` | 'hdftl' will be removed in a future release. There is no simple replacement for this. |
| `HYPERCUBE` | 'hypercube' will be removed in a future release. Use 'blockedImage' instead. |
| `BIGIMAGE` | 'bigimage' will be removed in a future release. Use 'blockedImage' instead. |
| `BIGIMAGEDS` | 'bigimageDatastore' will be removed in a future release. Use 'blockedImageDatastore' instead. |
| `LABELVOL` | 'labelvolshow' will be removed in a future release. Use 'labeloverlay3' instead. |
| `COMPATVOL` | 'volshow' will be removed in a future release. Use 'viewer3d' with 'volshow' instead. |
| `DEMLC` | 'emlc' will be removed in a future release. Use 'codegen' instead. |
| `DEMLMEX` | 'emlmex' will be removed in a future release. Use 'codegen' instead. |
| `COMMBI` | 'biterr' (legacy) will be removed in a future release. Use 'comm.ErrorRate' instead. |
| `PNZOM` | 'panzoom' will be removed in a future release. Use 'zoom' and 'pan' instead. |
| `SPTL` | 'sptool' will be removed in a future release. Use 'signalAnalyzer' instead. |
| `NDWT` | 'ndwt' will be removed in a future release. Use 'waveletTransform3' instead. |
| `INDWT` | 'indwt' will be removed in a future release. Use 'waveletTransform3' inverse instead. |
| `NDWT2` | 'ndwt2' will be removed in a future release. There is no simple replacement for this. |
| `INDWT2` | 'indwt2' will be removed in a future release. There is no simple replacement for this. |
| `WEBMP` | 'webmap' will be removed in a future release. Use geographic map visualizations instead. |
| `SERLL` | 'seriallist' will be removed in a future release. Use 'serialportlist' instead. |
| `RNG2BW` | 'range2bw' will be removed in a future release. Use 'rangeres2bw' instead. |
| `BW2RNG` | 'bw2range' will be removed in a future release. Use 'bw2rangeres' instead. |
| `DCSHELP` | 'docsearch' will be removed in a future release. There is no simple replacement for this. |
| `DFIGFLAG` | 'figflag' will be removed in a future release. There is no simple replacement for this. |
| `DPOPUP` | 'popupstr' will be removed in a future release. There is no simple replacement for this. |
| `ACTXC` | 'actxcontrol' will be removed in a future release. There is no simple replacement for this. |
| `ACTXL` | 'actxcontrollist' will be removed in a future release. There is no simple replacement for this. |
| `ACTXS` | 'actxcontrolselect' will be removed in a future release. There is no simple replacement for this. |
| `SOAPM` | 'createSoapMessage' will be removed in a future release. Use 'matlab.net.http' instead. |
| `SOAPS` | 'callSoapService' will be removed in a future release. Use 'matlab.net.http' instead. |
| `SOAPR` | 'parseSoapResponse' will be removed in a future release. Use 'matlab.net.http' instead. |
| `SOAPC` | 'createClassFromWsdl' will be removed in a future release. Use 'matlab.net.http' instead. |
| `VREDT` | 'vredit' will be removed in a future release. There is no simple replacement for this. |
| `VRPLY` | 'vrplay' will be removed in a future release. There is no simple replacement for this. |
| `VRCNVS` | 'vrcanvas' will be removed in a future release. There is no simple replacement for this. |
| `VRFIG` | 'vrfigure' will be removed in a future release. There is no simple replacement for this. |
| `VRVIEW` | 'vrview' will be removed in a future release. There is no simple replacement for this. |
| `VRWRLD` | 'vrworld' will be removed in a future release. There is no simple replacement for this. |
| `VRNDE` | 'vrnode' will be removed in a future release. There is no simple replacement for this. |
| `VRJOYSTK` | 'vrjoystick' will be removed in a future release. There is no simple replacement for this. |
| `VRSPCMOUSE` | 'vrspacemouse' will be removed in a future release. There is no simple replacement for this. |
| `SLRTBNCH` | 'slrtbench' will be removed in a future release. There is no simple replacement for this. |
| `RCSIIR` | 'rcosine' (IIR) will be removed in a future release. Use 'rcosdesign' instead. |
| `RCSFIR` | 'rcosine' (FIR) will be removed in a future release. Use 'rcosdesign' instead. |
| `G2B` | 'gray2bin' will be removed in a future release. There is no simple replacement for this. |
| `B2G` | 'bin2gray' will be removed in a future release. There is no simple replacement for this. |
| `EYESCOPE` | 'eyediagram' will be removed in a future release. Use 'comm.EyeDiagram' instead. |
| `DLDPCENC` | 'fec.ldpcenc' will be removed in a future release. Use 'comm.LDPCEncoder' instead. |
| `DLDPCDEC` | 'fec.ldpcdec' will be removed in a future release. Use 'comm.LDPCDecoder' instead. |
| `WEIBCDF` | 'weibcdf' will be removed in a future release. Use 'wblcdf' instead. |
| `WEIBFIT` | 'weibfit' will be removed in a future release. Use 'wblfit' instead. |
| `WEIBINV` | 'weibinv' will be removed in a future release. Use 'wblinv' instead. |
| `WEIBLIKE` | 'weiblike' will be removed in a future release. Use 'wbllike' instead. |
| `WEIBPDF` | 'weibpdf' will be removed in a future release. Use 'wblpdf' instead. |
| `WEIBPLOT` | 'weibplot' will be removed in a future release. Use 'wblplot' instead. |
| `WEIBRND` | 'weibrnd' will be removed in a future release. Use 'wblrnd' instead. |
| `WEIBSTAT` | 'weibstat' will be removed in a future release. Use 'wblstat' instead. |
| `OPCSERVER` | 'opcserverinfo' will be removed in a future release. There is no simple replacement for this. |
| `OPCDA` | 'opcda' will be removed in a future release. Use 'opcua' instead. |
| `OPCFIND` | 'opcfind' will be removed in a future release. There is no simple replacement for this. |
| `OPCRESET` | 'opcreset' will be removed in a future release. There is no simple replacement for this. |
| `OPCHELP` | 'opchelp' will be removed in a future release. There is no simple replacement for this. |
| `DSPBQF` | 'dsp.BiquadFilter' will be removed in a future release. Use 'dsp.SOSFilter' instead. |
| `DSPLPC` | 'dsp.LPC' will be removed in a future release. Use 'lpc' instead. |
| `DSPAVA` | 'dsp.ArrayVectorAdder' will be removed in a future release. Use built-in addition instead. |
| `DSPAVS` | 'dsp.ArrayVectorSubtractor' will be removed in a future release. Use built-in subtraction instead. |
| `DSPAVM` | 'dsp.ArrayVectorMultiplier' will be removed in a future release. Use built-in multiplication instead. |
| `DSPAVD` | 'dsp.ArrayVectorDivider' will be removed in a future release. Use built-in division instead. |
| `DSPCTR` | 'dsp.Counter' will be removed in a future release. There is no simple replacement for this. |
| `DSPKFT` | 'dsp.KalmanFilter' will be removed in a future release. There is no simple replacement for this. |
| `DSPNORM` | 'dsp.Normalizer' will be removed in a future release. Use 'normalize' instead. |
| `DSPAUDIOREC` | 'dsp.AudioRecorder' will be removed in a future release. Use 'audioDeviceReader' instead. |
| `DSPAUDIOPLAY` | 'dsp.AudioPlayer' will be removed in a future release. Use 'audioDeviceWriter' instead. |
| `DSPBUFFER` | 'dsp.Buffer' will be removed in a future release. Use 'dsp.AsyncBuffer' instead. |
| `DSPHIST` | 'dsp.Histogram' will be removed in a future release. Use 'histogram' instead. |
| `DSPMAX` | 'dsp.Maximum' will be removed in a future release. Use 'movmax' or 'max' instead. |
| `DSPMIN` | 'dsp.Minimum' will be removed in a future release. Use 'movmin' or 'min' instead. |
| `DSPMEAN` | 'dsp.Mean' will be removed in a future release. Use 'movmean' or 'mean' instead. |
| `DSPMEDIAN` | 'dsp.Median' will be removed in a future release. Use 'movmedian' or 'median' instead. |
| `DSPRMS` | 'dsp.RMS' will be removed in a future release. Use 'rms' instead. |
| `DSPSTD` | 'dsp.StandardDeviation' will be removed in a future release. Use 'movstd' or 'std' instead. |
| `DSPVAR` | 'dsp.Variance' will be removed in a future release. Use 'movvar' or 'var' instead. |
| `DSPCUMPROD` | 'dsp.CumulativeProduct' will be removed in a future release. Use 'cumprod' instead. |
| `DSPCUMSUM` | 'dsp.CumulativeSum' will be removed in a future release. Use 'cumsum' instead. |
| `DSPINTERP` | 'dsp.Interpolator' will be removed in a future release. Use 'interp1' instead. |
| `DSPCONV` | 'dsp.Convolver' will be removed in a future release. Use 'conv' instead. |
| `DSPAUTOCORR` | 'dsp.Autocorrelator' will be removed in a future release. Use 'xcorr' instead. |
| `DSPXCORR` | 'dsp.Crosscorrelator' will be removed in a future release. Use 'xcorr' instead. |
| `DSPLDL` | 'dsp.LDLFactor' will be removed in a future release. Use 'ldl' instead. |
| `DSPLU` | 'dsp.LUFactor' will be removed in a future release. Use 'lu' instead. |
| `DSPLEVINSON` | 'dsp.LevinsonSolver' will be removed in a future release. Use 'levinson' instead. |
| `DSPDELAYLINE` | 'dsp.DelayLine' will be removed in a future release. Use 'dsp.AsyncBuffer' instead. |
| `DSPWIN` | 'dsp.Window' will be removed in a future release. Use 'window functions' directly instead. |
| `DSPSA` | 'dsp.SpectrumAnalyzer' scope syntax will be removed in a future release. Use 'spectrumAnalyzer' instead. |
| `CMLLD` | 'ClassificationModel.logLoss' will be removed in a future release. There is no simple replacement for this. |
| `CMLSV` | 'ClassificationModel.logSV' will be removed in a future release. There is no simple replacement for this. |
| `SESSION` | 'daq.createSession' will be removed in a future release. Use 'daqlist' and 'daq' interface instead. |
| `DRYICE` | 'dryice' will be removed in a future release. There is no simple replacement for this. |
| `PERFREF` | 'perfcurve' old syntax will be removed in a future release. Use updated syntax instead. |
| `YOLOV` | 'yolov2ObjectDetector' will be removed in a future release. Use 'yoloxObjectDetector' instead. |
| `DDTRD` | 'dtree' will be removed in a future release. There is no simple replacement for this. |
| `IMPIVD` | Malformed import argument VAR_NAME will not be supported in a future release. |
| `IMPKEY` | Importing VAR_NAME will not be supported in a future release because VAR_NAME is a reserved word. |
| `REDEFGI` | Declaring an input or output variable to be global might not be supported in a future release. |
| `REDEFGG` | Declaring a variable to be global more than once might not be supported in a future release. |
| `MCPDC` | Specifying both the 'Constant' and 'Dependent' attributes on the same property is not supported. |
| `NOV6` | 'v6' will be removed in a future release. There is no simple replacement for this. |
| `V6ON` | USEV6PLOTAPI('on') will be removed in a future release. There is no simple replacement for this. |
| `PSTAT` | The 'Static' attribute on properties has been removed. Use the 'Constant' attribute instead. |
| `FGREN` | 'Renderer' will be removed in a future release and currently has no effect. There is no simple replacement for this. |
| `FGREM` | 'RendererMode' will be removed in a future release and currently has no effect. There is no simple replacement for this. |
| `FROPT` | '-dill' has been removed. Use Encapsulated PostScript instead. |
| `FROPTX` | '-adobecset' has been removed. There is no simple replacement for this. |
| `DSPIDF` | 'DirectFeedthrough' property of 'dsp.VariableIntegerDelay' class has been removed. |
| `DSPFDF` | 'DirectFeedthrough' property of 'dsp.VariableFractionalDelay' class has been removed. |
| `ATVIZW` | The 'Visible' attribute has been removed. Use the '~Hidden' attribute instead or omit the attribute since 'Hidden' is false by default. |
| `MCGCP` | Defining a get method for a constant property is not supported. |
| `MCATP` | Using an @ sign to specify a class property restriction is unsupported and has been removed. Use property validation syntax instead. |
| `NCHKNO` | NARGOUTCHK using more than two inputs will be removed in a future release. There is no simple replacement for this. |
| `HESST` | 'InitialHessType' has been removed. There is no simple replacement for this. |
| `HESSM` | 'InitialHessMatrix' has been removed. There is no simple replacement for this. |
| `BUFSIZE` | Option 'Bufsize' has been removed. Manual buffering in 'textscan' is no longer needed. |
| `FEGLO` | 'global' has been removed. There is no simple replacement for this. |
| `SMPLMODE` | The property 'FrameBasedProcessing' has been removed. |
| `RESOU` | 'resources' is a reserved folder. Running MATLAB files located in a folder named 'resources' is not supported. |
| `TTSMP` | 'SamplingRate' has been removed. Use 'SampleRate' instead. |
| `FINSI` | Support for 'inputname' in a script has been removed. |
| `FINSNI` | Support for 'nargin' in a script has been removed. |
| `FINSNO` | Support for 'nargout' in a script has been removed. |
| `DISGVER` | 'matlab.graphics.internal.isGraphicsVersion1' has been removed. Use 'verLessThan('matlab','8.4.0')' instead. |
| `DNDLA` | 'discard' has been removed. There is no simple replacement for this. |
| `EITYCN` | feature('EightyColumns') and feature('EightyColumns', VALUE) are unsupported and have been removed. With appropriate code changes, use 'settings' object instead. |
| `REPUDD` | Classes defined using schema.m files are no longer supported. Use MATLAB Classes defined using the classdef keyword instead. |
| `RTWHWDR` | 'RTW.HWDeviceRegistry' is unsupported and has been removed. The replacement strategy can be found in MATLAB documentation. |
| `INVHCRM` | The 'InvertHardCopy' property will be removed in a future release and currently has no effect. |
| `DINVHCRM` | The 'defaultFigureInvertHardCopy' setting will be removed in a future release and currently has no effect. |
| `CHKGR` | The 'CheckGradients' option has been removed from 'optimoptions'. With appropriate code changes, use the 'checkGradients' function instead. |
| `LINPROGS` | 'simplex' algorithm has been removed. With appropriate code changes, set 'Algorithm' value to 'interior-point' or 'dual-simplex' instead. |
| `LSRET` | 'LaserReturns' has been removed. Use 'LaserReturn' instead, which is a direct replacement. |
| `RAYNR` | Input argument 'NumReflections' has been removed. Use 'MaxNumReflections' property of a ray tracing propagation model object instead. |
| `JAPIMATHWORKS` | 'com.mathworks' namespace and sub namespaces will be removed in a future release. There is no simple replacement for this. |
| `IMCLASS` | In a future release, 'ismethod' will treat a string or character vector in its first input as a 'string' or 'char' class object. |
| `WEBREMOVE` | The 'web' function does not return a handle or URL for pages that open in the system browser. Use 'stat = web(___, '-browser')' instead. |
| `TCPC` | 'tcpip' with 'client' as a 'NetworkRole' will be removed in a future release. With appropriate code changes, use 'tcpclient' instead. |
| `TCPS` | 'tcpip' with 'server' as a 'NetworkRole' will be removed in a future release. With appropriate code changes, use 'tcpserver' instead. |
| `QAMDEPM` | 'qammod' no longer accepts the initial phase of a signal. |
| `QAMDEPD` | 'qamdemod' no longer accepts the initial phase of a signal. |
| `GETERR` | The 'ErrorMessage' property has been removed. At the command line, use 'MException.last' instead. |
| `SETERR` | The 'ErrorMessage' property has been removed. There is no simple replacement for this. |
| `MAPVW` | 'mapview' will be removed in a future release. Use geographic or map axes instead. |
| `MAPTOOL` | 'maptool' will be removed in a future release. There is no simple replacement for this. |
| `DFEATUREPARAM1` | 'UseHG2' has been removed. With appropriate code changes, use '~verLessThan('matlab','8.4.0')' instead. |
| `DFEATUREPARAM2` | 'HGUsingMATLABClasses' has been removed. With appropriate code changes, use '~verLessThan('matlab','8.4.0')' instead. |
| `DILEVAL` | 'dicomLookupEval' will be removed in a future release. There is no simple replacement for this. |
| `DSPPEAKS` | 'dsp.PeakFinder' will be removed in a future release. Use 'findpeaks' instead. |
| `DSPPEAK2PEAK` | 'dsp.PeakToPeak' will be removed in a future release. Use 'peak2peak' instead. |
| `DSP_LINKS` | 'dsp.SignalSource' will be removed in a future release. There is no simple replacement for this. |
| `MCASCADE` | 'mfilt.cascade' will be removed in a future release. Use 'dsp.FilterCascade' instead. |
| `FDESPARAMEQ` | 'fdesign.parameq' will be removed in a future release. Use 'designParamEQ' instead. |
| `FDESOCTAVE` | 'fdesign.octave' will be removed in a future release. Use 'designOctaveFilter' instead. |
| `FDESWEIGHT` | 'fdesign.arbmagnphase' will be removed in a future release. Use 'designfilt' instead. |
| `AFLMS` | 'adaptfilt.lms' will be removed in a future release. Use 'dsp.LMSFilter' instead. |
| `AFNLMS` | 'adaptfilt.nlms' will be removed in a future release. Use 'dsp.LMSFilter' with NLMS algorithm instead. |
| `AFRLS` | 'adaptfilt.rls' will be removed in a future release. Use 'dsp.RLSFilter' instead. |
| `AFBLMS` | 'adaptfilt.blms' will be removed in a future release. Use 'dsp.BlockLMSFilter' instead. |
| `DNANMEAN` | 'nanmean' from Statistics Toolbox is not recommended. Use 'mean' with 'omitnan' option instead. |
| `DNANSTD` | 'nanstd' from Statistics Toolbox is not recommended. Use 'std' with 'omitnan' option instead. |
| `DNANVAR` | 'nanvar' from Statistics Toolbox is not recommended. Use 'var' with 'omitnan' option instead. |
| `DNANMEDIAN` | 'nanmedian' from Statistics Toolbox is not recommended. Use 'median' with 'omitnan' option instead. |
| `DNANMIN` | 'nanmin' from Statistics Toolbox is not recommended. Use 'min' with 'omitnan' option instead. |
| `DNANMAX` | 'nanmax' from Statistics Toolbox is not recommended. Use 'max' with 'omitnan' option instead. |
| `DNANSUM` | 'nansum' from Statistics Toolbox is not recommended. Use 'sum' with 'omitnan' option instead. |
| `DNANCOV` | 'nancov' from Statistics Toolbox is not recommended. Use covariance computation with 'omitrows' option instead. |
| `DWAVREAD` | 'wavread' has been removed. Use 'audioread' instead. |
| `DWAVWRITE` | 'wavwrite' has been removed. Use 'audiowrite' instead. |
| `DAUREAD` | 'auread' has been removed. Use 'audioread' instead. |
| `DAUWRITE` | 'auwrite' has been removed. Use 'audiowrite' instead. |
| `DQUAD2` | 'quad' is not recommended. Use 'integral' instead. |
| `DQUADL2` | 'quadl' is not recommended. Use 'integral' instead. |
| `DQUADV2` | 'quadv' is not recommended. Use 'integral' with 'ArrayValued' option instead. |
| `DDBLQD2` | 'dblquad' is not recommended. Use 'integral2' instead. |
| `DTRIQD2` | 'triplequad' is not recommended. Use 'integral3' instead. |
| `DEZPLOT` | 'ezplot' is not recommended. Use 'fplot' instead. |
| `DEZMESH` | 'ezmesh' is not recommended. Use 'fmesh' instead. |
| `DEZSURF` | 'ezsurf' is not recommended. Use 'fsurf' instead. |
| `DEZCONTOUR` | 'ezcontour' is not recommended. Use 'fcontour' instead. |
| `DEZPOLAR` | 'ezpolar' is not recommended. Use 'polarplot' instead. |
| `DEZPLOT3` | 'ezplot3' is not recommended. Use 'fplot3' instead. |
| `DEZSURFC` | 'ezsurfc' is not recommended. Use 'fsurf' instead. |
| `DEZMESHC` | 'ezmeshc' is not recommended. Use 'fmesh' instead. |
| `DEZCONTOURF` | 'ezcontourf' is not recommended. Use 'fcontour' instead. |
| `DPLOTYY` | 'plotyy' is not recommended. Use 'yyaxis' instead. |
| `DPOLAR` | 'polar' is not recommended. Use 'polarplot' instead. |
| `DCOMPASS` | 'compass' is not recommended. Use 'polarplot' instead. |
| `DROSE` | 'rose' is not recommended. Use 'polarhistogram' instead. |
| `DHIST` | 'hist' is not recommended. Use 'histogram' instead. |
| `DHISTC` | 'histc' is not recommended. Use 'histcounts' instead. |
| `DCAXIS` | 'caxis' is not recommended. Use 'clim' instead. |
| `DHOLDALL` | 'hold all' is not recommended. Use 'hold on' instead. |
| `DCSVREAD` | 'csvread' is not recommended. Use 'readmatrix' instead. |
| `DCSVWRITE` | 'csvwrite' is not recommended. Use 'writematrix' instead. |
| `DDLMREAD` | 'dlmread' is not recommended. Use 'readmatrix' instead. |
| `DDLMWRITE` | 'dlmwrite' is not recommended. Use 'writematrix' instead. |
| `DXLSREAD` | 'xlsread' is not recommended. Use 'readtable', 'readmatrix', or 'readcell' instead. |
| `DXLSWRITE` | 'xlswrite' is not recommended. Use 'writetable', 'writematrix', or 'writecell' instead. |
| `DTEXTREAD` | 'textread' has been removed. Use 'textscan' instead. |
| `DSTRREAD` | 'strread' has been removed. Use 'textscan' instead. |
| `DURLREAD` | 'urlread' is not recommended. Use 'webread' instead. |
| `DURLWRITE` | 'urlwrite' is not recommended. Use 'websave' instead. |
| `DH5READ` | 'hdf5read' has been removed. Use 'h5read' instead. |
| `DH5WRITE` | 'hdf5write' has been removed. Use 'h5write' instead. |
| `DH5INFO` | 'hdf5info' has been removed. Use 'h5info' instead. |
| `DIM2BW` | 'im2bw' is not recommended. Use 'imbinarize' instead. |
| `DIMFREEHAND` | 'imfreehand' is not recommended. Use 'drawfreehand' instead. |
| `DIMRECT` | 'imrect' is not recommended. Use 'drawrectangle' instead. |
| `DIMLINE` | 'imline' is not recommended. Use 'drawline' instead. |
| `DIMPOINT` | 'impoint' is not recommended. Use 'drawpoint' instead. |
| `DIMPOLY` | 'impoly' is not recommended. Use 'drawpolygon' instead. |
| `DIMELLIPSE` | 'imellipse' is not recommended. Use 'drawellipse' instead. |
| `DROIFILL` | 'roifill' is not recommended. Use 'regionfill' instead. |
| `DDATASET` | 'dataset' is not recommended. Use 'table' instead. |
| `DNOMINAL` | 'nominal' has been removed. Use 'categorical' instead. |
| `DORDINAL` | 'ordinal' has been removed. Use 'categorical' with 'Ordinal' option instead. |
| `DDATENUM` | 'datenum' is not recommended. Use 'datetime' instead. |
| `DDATESTR` | 'datestr' is not recommended. Use 'string' or 'char' on datetime objects instead. |
| `DDATEVEC` | 'datevec' is not recommended. Use datetime properties (Year, Month, Day, etc.) instead. |
| `DCLOCK` | 'clock' is not recommended. Use 'datetime(\ |
| `DDATE` | 'date' is not recommended. Use 'datetime(\ |
| `DNOW` | 'now' is not recommended. Use 'datetime(\ |
| `DTODAY` | 'today' (datenum) is not recommended. Use 'datetime(\ |
| `DTIC` | Using 'tic'/'toc' for profiling is not recommended. Use 'timeit' for accurate timing. |
| `DDEBLANK` | 'deblank' is not recommended. Use 'strtrim' or 'strip' instead. |
| `DFINDSTR` | 'findstr' has been removed. Use 'strfind' or 'contains' instead. |
| `DSTRMATCH` | 'strmatch' has been removed. Use 'startsWith', 'matches', or 'strcmp' instead. |
| `DSTRVCAT` | 'strvcat' has been removed. Use 'char' or string arrays instead. |
| `DGENVARNAME` | 'genvarname' has been removed. Use 'matlab.lang.makeValidName' instead. |
| `DGUIDE` | 'guide' (GUIDE) is not recommended. Use App Designer instead. |
| `DINPUTDLG` | 'inputdlg' is not recommended for new apps. Use App Designer UI components instead. |
| `DWARNDLG` | 'warndlg' is not recommended. Use 'uialert' instead. |
| `DERRORDLG` | 'errordlg' is not recommended. Use 'uialert' instead. |
| `DMSGBOX` | 'msgbox' is not recommended. Use 'uialert' or 'uiconfirm' instead. |
| `DQUESTDLG` | 'questdlg' is not recommended. Use 'uiconfirm' instead. |
| `DSERIAL` | 'serial' is not recommended. Use 'serialport' instead. |
| `DGPIB2` | 'gpib' is not recommended. Use 'visadev' instead. |
| `DVISA2` | 'visa' is not recommended. Use 'visadev' instead. |
| `DTCPIP2` | 'tcpip' is not recommended. Use 'tcpclient' or 'tcpserver' instead. |
| `DUDP2` | 'udp' is not recommended. Use 'udpport' instead. |
| `DFMINUNC_OPT` | 'optimset' is not recommended for newer solvers. Use 'optimoptions' instead. |
| `DADDPREF` | 'addpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| `DGETPREF` | 'getpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| `DSETPREF` | 'setpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| `DRMPREF` | 'rmpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| `DISPREF` | 'ispref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| `DINLINE2` | 'inline' has been removed. Use anonymous functions (@(x) ...) instead. |
| `DSYM2POLY` | 'sym2poly' is not recommended. Use 'coeffs' instead. |
| `DFLIPUD` | For vectors, 'flipud' can be replaced by 'flip'. |
| `DFLIPLR` | For vectors, 'fliplr' can be replaced by 'flip'. |
| `DISDIR` | 'isdir' is not recommended. Use 'isfolder' instead. |
| `DFILEATTRIB` | 'fileattrib' is not recommended. Use 'dir' with file attributes instead. |
| `DVERLESSTHAN` | 'verLessThan' is not recommended. Use 'isMATLABReleaseOlderThan' instead. |
| `DNARGCHK` | 'nargchk' has been removed. Use 'narginchk' instead. |
| `DNARGOUTCHK` | The error-string form of 'nargoutchk' is not recommended. Use the narginchk-style calling form instead. |
| `DCOMBNK` | 'combnk' is not recommended. Use 'nchoosek' or 'combinations' instead. |
| `DMATLABPOOL` | 'matlabpool' has been removed. Use 'parpool' instead. |
| `DDISTCOMP` | Older 'distributed' syntax is not recommended. Use newer parallel computing patterns. |
| `DLABINDEX` | 'labindex' is not recommended. Use 'spmdIndex' instead. |
| `DNUMLABS` | 'numlabs' is not recommended. Use 'spmdSize' instead. |
| `DLABBARRIER` | 'labBarrier' is not recommended. Use 'spmdBarrier' instead. |
| `DLABSEND` | 'labSend' is not recommended. Use 'spmdSend' instead. |
| `DLABRECEIVE` | 'labReceive' is not recommended. Use 'spmdReceive' instead. |
| `DLABBROADCAST` | 'labBroadcast' is not recommended. Use 'spmdBroadcast' instead. |
| `DLABSENDRECEIVE` | 'labSendReceive' is not recommended. Use 'spmdSendReceive' instead. |
| `DLABPROBE` | 'labProbe' is not recommended. Use 'spmdProbe' instead. |
| `DBARTLETT` | 'bartlett' will be removed. Use 'barthannwin' or window functions from Signal Processing Toolbox. |
| `DBLACKMANHARRIS` | 'blackmanharris' is not recommended. Use 'blackmanharris' from Signal Processing Toolbox with updated syntax. |
| `DBOHMANWIN` | 'bohmanwin' window function syntax has been updated. |
| `DGAUSSWIN` | 'gausswin' window function syntax has been updated. |
| `DKAISER` | Use updated 'kaiser' window syntax. |
| `DHAMMING` | Use updated 'hamming' window syntax. |
| `DHANNING` | 'hanning' is not recommended. Use 'hann' instead. |
| `DPBURG` | 'pburg' function syntax has been updated. |
| `DPCOV` | 'pcov' function syntax has been updated. |
| `DPMTM` | 'pmtm' function syntax has been updated. |
| `DPMUSIC` | 'pmusic' function syntax has been updated. |
| `DPWELCH` | 'pwelch' function syntax has been updated. |
| `DPYULEAR` | 'pyulear' function syntax has been updated. |
| `AXSTATE` | 'axis('state')' has been removed. With appropriate code changes, use 'XLimMode', 'YLimMode', 'ZLimMode', 'Visible', 'XDir', and 'YDir' properties of an axes object instead. |
| `FDDECI1` | The 'Raised Cosine' response method of 'fdesign.decimator' object has been removed. With appropriate code changes use 'comm.RaisedCosineReceiveFilter' object instead. |
| `FDDECI2` | The 'Square Root Raised Cosine' response method of 'fdesign.decimator' object has been removed. With appropriate code changes use 'comm.RaisedCosineReceiveFilter' object instead. |
| `FDINPO1` | The 'Raised Cosine' response method of 'fdesign.interpolator' object has been removed. With appropriate code changes use 'comm.RaisedCosineTransmitFilter' object instead. |
| `FDINPO2` | The 'Square Root Raised Cosine' response method of 'fdesign.interpolator' object has been removed. With appropriate code changes use 'comm.RaisedCosineTransmitFilter' object instead. |
| `WAVMENU` | 'wavemenu' has been removed. With appropriate code changes, use Signal Multiresolution Analyzer, Wavelet Image Analyzer, Wavelet Signal Analyzer, Wavelet Signal Denoiser, or Wavelet Time-Frequency Analyzer instead. |
| `WAVLAZ` | 'waveletAnalyzer' has been removed. With appropriate code changes, use Signal Multiresolution Analyzer, Wavelet Image Analyzer, Wavelet Signal Analyzer, Wavelet Signal Denoiser, or Wavelet Time-Frequency Analyzer instead. |
| `PRJET` | 'project' has been removed. With appropriate code changes, use 'projfwd' instead. |
| `RETPRI` | Positional syntax for optional arguments has been removed from 'ret2price'. Use name-value pairs instead. |
| `PRIRET` | Positional syntax for optional arguments has been removed from 'price2ret'. Use name-value pairs instead. |
| `ACORR` | Positional syntax for optional arguments has been removed from 'autocorr'. Use name-value pairs instead. |
| `PCORR` | Positional syntax for optional arguments has been removed from 'parcorr'. Use name-value pairs instead. |
| `XCORR` | Positional syntax for optional arguments has been removed from 'crosscorr'. Use name-value pairs instead. |
| `HPFLTR` | Positional syntax for optional arguments has been removed from 'hpfilter'. Use name-value pairs instead. |
| `H5PGET` | 'H5P.get_dxpl_multi' has been removed. There is no simple replacement for this. |
| `H5PSET` | 'H5P.set_dxpl_multi' has been removed. There is no simple replacement for this. |
| `DGETST` | 'getstatus' will be removed in a future release. There is no simple replacement for this. |
| `DMENUL` | 'menulabel' will be removed in a future release. There is no simple replacement for this. |
| `DUIGET` | 'uigettoolbar' will be removed in a future release. There is no simple replacement for this. |
| `AVRREM` | The 'Aero.VirtualRealityAnimation' class no longer creates visualizations and will be removed in a future release. With appropriate code changes, use sim3d classes to create and view a 3D environment instead. |
| `DAVIINF` | 'aviinfo' will be removed in a future release. With appropriate code changes, use 'VideoReader' instead. |
| `DAFINF` | 'avifinfo' will be removed in a future release. With appropriate code changes, use 'VideoReader' instead. |
| `COMMCCDF` | 'comm.CCDF' has been removed. With appropriate code changes, use 'powermeter' instead. |
| `COMMIB` | 'comm.IntegerToBit' has been removed. With appropriate code changes, use 'int2bit' instead. |
| `COMMSCOPESP` | 'commscope.ScatterPlot' has been removed. With appropriate code changes, use 'comm.ConstellationDiagram' instead. |
| `COMMBSC` | 'comm.BinarySymmetricChannel' has been removed. With appropriate code changes, use 'BSC' instead. |
| `RLCHN` | 'rayleighchan' has been removed. With appropriate code changes, use 'comm.RayleighChannel' instead. |
| `RICHN` | 'ricianchan' has been removed. With appropriate code changes, use 'comm.RicianChannel' instead. |
| `LEGCHN` | 'legacychannelsim' has been removed. There is no simple replacement for this. |
| `DOPJKS` | 'doppler.jakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPRJKS` | 'doppler.rjakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPAJKS` | 'doppler.ajakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPFLT` | 'doppler.flat' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPBLL` | 'doppler.bell' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPRNDD` | 'doppler.rounded' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPGSS` | 'doppler.gaussian' has been removed. With appropriate code changes, use 'doppler' instead. |
| `DOPBGSS` | 'doppler.bigaussian' has been removed. With appropriate code changes, use 'doppler' instead. |
| `COMMPSKC` | 'comm.PSKCoarseFrequencyEstimator' has been removed. With appropriate code changes, use 'comm.CoarseFrequencyCompensator' instead. |
| `COMMQAMC` | 'comm.QAMCoarseFrequencyEstimator' has been removed. With appropriate code changes, use 'comm.CoarseFrequencyCompensator' instead. |
| `DEVM` | 'commmeasure.EVM' has been removed. With appropriate code changes, use 'comm.EVM' instead. |
| `DMER` | 'commmeasure.MER' has been removed. With appropriate code changes, use 'comm.MER' instead. |
| `DACPR` | 'commmeasure.ACPR' has been removed. With appropriate code changes, use 'comm.ACPR' instead. |
| `CRCGE` | 'crc.generator' has been removed. With appropriate code changes, use 'comm.CRCGenerator' instead. |
| `CRCDE` | 'crc.detector' has been removed. With appropriate code changes, use 'comm.CRCDetector' instead. |
| `COMMCRCGEN` | 'comm.CRCGenerator' will be removed in a future release. With appropriate code changes, use 'crcGenerate' instead. |
| `COMMCRCDET` | 'comm.CRCDetector' will be removed in a future release. With appropriate code changes, use 'crcDetect' instead. |
| `COMMDVBS2LDPC` | 'dvbs2ldpc' will be removed in a future release. With appropriate code changes, use 'dvbsLDPCPCM' instead. |
| `CMDFE` | 'dfe' has been removed. With appropriate code changes, use 'comm.DecisionFeedbackEqualizer' instead. |
| `CMDFEEQ` | 'equalizer.dfe' has been removed. With appropriate code changes, use 'comm.DecisionFeedbackEqualizer' instead. |
| `CMLRQ` | 'lineareq' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' instead. |
| `CMLRQEQ` | 'equalizer.lineareq' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' instead. |
| `CMLMSAD` | 'adaptalg.lms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMRLSAD` | 'adaptalg.rls' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMCMAAD` | 'adaptalg.cma' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMSLMSAD` | 'adaptalg.signlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMVLMSAD` | 'adaptalg.varlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMNLMSAD` | 'adaptalg.normlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMLMS` | 'lms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMRLS` | 'rls' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMCMA` | 'cma' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMSLMS` | 'signlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMVLMS` | 'varlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMNLMS` | 'normlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `CMEQU` | 'equalize' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| `DBHENC` | 'fec.bchenc' has been removed. With appropriate code changes, use 'comm.BCHEncoder' instead. |
| `DBCHDEC` | 'fec.bchdec' has been removed. With appropriate code changes, use 'comm.BCHDecoder' instead. |
| `DRSENC` | 'fec.rsenc' has been removed. With appropriate code changes, use 'comm.RSEncoder' instead. |
| `DRSDEC` | 'fec.rsdec' has been removed. With appropriate code changes, use 'comm.RSDecoder' instead. |
| `COMMGPUAWGN` | 'comm.gpu.AWGNChannel' will be removed in a future release. With appropriate code changes, use 'awgn' instead. |
| `COMMGPULDPCDEC` | 'comm.gpu.LDPCDecoder' will be removed in a future release. With appropriate code changes, use 'ldpcDecode' instead. |
| `COMMGPUBLKINTRLV` | 'comm.gpu.BlockInterleaver' will be removed in a future release. With appropriate code changes, use 'intrlv' instead. |
| `COMMGPUBLKDEINTRLV` | 'comm.gpu.BlockDeinterleaver' will be removed in a future release. With appropriate code changes, use 'deintrlv' instead. |
| `COMMGPUCONVINTRLV` | 'comm.gpu.ConvolutionalInterleaver' will be removed in a future release. With appropriate code changes, use 'convintrlv' instead. |
| `COMMGPUCONVDEINTRLV` | 'comm.gpu.ConvolutionalDeinterleaver' will be removed in a future release. With appropriate code changes, use 'convdeintrlv' instead. |
| `COMMGPUPSKMODULATOR` | 'comm.gpu.PSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| `COMMGPUPSKDEMODULATOR` | 'comm.gpu.PSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| `LDPCE` | 'comm.LDPCEncoder' has been removed. With appropriate code changes, use 'ldpcEncode' instead. |
| `LDPCD` | 'comm.LDPCDecoder' has been removed. With appropriate code changes, use 'ldpcDecode' instead. |
| `COMMLMC` | 'comm.LTEMIMOChannel' has been removed. With appropriate code changes, use 'comm.MIMOChannel' instead. |
| `QMOD` | 'modem.qammod' has been removed. With appropriate code changes, use 'qammod' instead. |
| `QDEMOD` | 'modem.qamdemod' has been removed. With appropriate code changes, use 'qamdemod' instead. |
| `DMOD` | 'modem.dpskmod' has been removed. With appropriate code changes, use 'comm.DPSKModulator' instead. |
| `DDEMOD` | 'modem.dpskdemod' has been removed. With appropriate code changes, use 'comm.DPSKDemodulator' instead. |
| `OMOD` | 'modem.oqpskmod' has been removed. With appropriate code changes, use 'comm.OQPSKModulator' instead. |
| `ODEMOD` | 'modem.oqpskdemod' has been removed. With appropriate code changes, use 'comm.OQPSKDemodulator' instead. |
| `PAMOD` | 'modem.pammod' has been removed. With appropriate code changes, use 'pammod' instead. |
| `PADEMOD` | 'modem.pamdemod' has been removed. With appropriate code changes, use 'pamdemod' instead. |
| `MMOD` | 'modem.mskmod' has been removed. With appropriate code changes, use 'comm.MSKModulator' or 'mskmod' instead. |
| `MDEMOD` | 'modem.mskdemod' has been removed. With appropriate code changes, use 'comm.MSKDemodulator' or 'mskdemod' instead. |
| `GMOD` | 'modem.genqammod' has been removed. With appropriate code changes, use 'genqammod' or 'comm.GeneralQAMModulator' instead. |
| `GDEMOD` | 'modem.genqamdemod' has been removed. With appropriate code changes, use 'genqamdemod' or 'comm.GeneralQAMDemodulator' instead. |
| `PSMOD` | 'modem.pskmod' has been removed. With appropriate code changes, use 'pskmod' instead. |
| `PSDEMOD` | 'modem.pskdemod' has been removed. With appropriate code changes, use 'pskdemod' instead. |
| `OQPMOD` | 'oqpskmod' has been removed. With appropriate code changes, use 'comm.OQPSKModulator' instead. |
| `OQPDEM` | 'oqpskdemod' has been removed. With appropriate code changes, use 'comm.OQPSKDemodulator' instead. |
| `COMMPSKMSO` | 'comm.PSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| `COMMPSKDSO` | 'comm.PSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| `COMMBPSKMSO` | 'comm.BPSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| `COMMBPSKDSO` | 'comm.BPSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| `COMMQPSKMSO` | 'comm.QPSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| `COMMQPSKDSO` | 'comm.QPSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| `CMPSKCPS` | 'comm.PSKCarrierPhaseSynchronizer' has been removed. With appropriate code changes, use 'comm.CarrierSynchronizer' instead. |
| `COMMQAMM` | 'comm.RectangularQAMModulator' has been removed. With appropriate code changes, use 'qammod' instead. |
| `COMMQAMD` | 'comm.RectangularQAMDemodulator' has been removed. With appropriate code changes, use 'qamdemod' instead. |
| `CMELGTS` | 'comm.EarlyLateGateTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| `CMGTS` | 'comm.GardnerTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| `CMMMTS` | 'comm.MuellerMullerTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| `ALDEINT` | 'comm.AlgebraicDeinterleaver' has been removed. With appropriate code changes, use 'algdeintrlv' instead. |
| `ALINT` | 'comm.AlgebraicInterleaver' has been removed. With appropriate code changes, use 'algintrlv' instead. |
| `BLKDEINT` | 'comm.BlockDeinterleaver' has been removed. With appropriate code changes, use 'deintrlv' instead. |
| `BLKINT` | 'comm.BlockInterleaver' has been removed. With appropriate code changes, use 'intrlv' instead. |
| `MATDEINT` | 'comm.MatrixDeinterleaver' has been removed. With appropriate code changes, use 'matdeintrlv' instead. |
| `MATINT` | 'comm.MatrixInterleaver' has been removed. With appropriate code changes, use 'matintrlv' instead. |
| `MATHSDEINT` | 'comm.MatrixHelicalScanDeinterleaver' has been removed. With appropriate code changes, use 'helscandeintrlv' instead. |
| `MATHSINT` | 'comm.MatrixHelicalScanInterleaver' has been removed. With appropriate code changes, use 'helscanintrlv' instead. |
| `ZADOF` | 'lteZadoffChuSeq' has been removed. Use 'zadoffChuSeq' instead, which is a direct replacement. |
| `CMCPMCPS` | 'comm.CPMCarrierPhaseSynchronizer' has been removed. With appropriate code changes, use 'comm.CarrierSynchronizer' instead. |
| `SESSIONR` | 'daq.reset' will be removed in a future release. Use 'daqreset' instead, which is a direct replacement. |
| `SESSIONGD` | 'daq.getDevices' will be removed in a future release. With appropriate code changes, use 'daqlist' instead. |
| `SESSIONGV` | 'daq.getVendors' will be removed in a future release. Use 'daqvendorlist' instead, which is a direct replacement. |
| `AFBLMSFFT` | 'adaptfilt.blmsfft' has been removed. There is no simple replacement for this. |
| `AFADJLMS` | 'adaptfilt.adjlms' has been removed. There is no simple replacement for this. |
| `AFDLMS` | 'adaptfilt.dlms' has been removed. There is no simple replacement for this. |
| `AFPBFDAF` | 'adaptfilt.pbfdaf' has been removed. There is no simple replacement for this. |
| `AFPBUFDAF` | 'adaptfilt.pbufdaf' has been removed. There is no simple replacement for this. |
| `AFTDAFDCT` | 'adaptfilt.tdafdct' has been removed. There is no simple replacement for this. |
| `AFTFAFDFT` | 'adaptfilt.tfafdft' has been removed. There is no simple replacement for this. |
| `AFSE` | 'adaptfilt.se' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| `AFSD` | 'adaptfilt.sd' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| `AFSS` | 'adaptfilt.ss' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| `AFQRDRLS` | 'adaptfilt.qrdrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| `AFSWRLS` | 'adaptfilt.swrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| `AFHRLS` | 'adaptfilt.hrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| `AFHSWRLS` | 'adaptfilt.hswrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| `AFSWFTF` | 'adaptfilt.swftf' has been removed. With appropriate code changes, use 'dsp.FastTransversalFilter' instead. |
| `AFFTF` | 'adaptfilt.ftf' has been removed. With appropriate code changes, use 'dsp.FastTransversalFilter' instead. |
| `AFAP` | 'adaptfilt.ap' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| `AFAPRU` | 'adaptfilt.apru' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| `AFBAP` | 'adaptfilt.bap' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| `AFGAL` | 'adaptfilt.gal' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| `AFLSL` | 'adaptfilt.lsl' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| `AFQRDLSL` | 'adaptfilt.qrdlsl' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| `AFFILTXLMS` | 'adaptfilt.filtxlms' has been removed. With appropriate code changes, use 'dsp.FilteredXLMSFilter' instead. |
| `AFFDAF` | 'adaptfilt.fdaf' has been removed. With appropriate code changes, use 'dsp.FrequencyDomainAdaptiveFilter' instead. |
| `AFUFDAF` | 'adaptfilt.ufdaf' has been removed. With appropriate code changes, use 'dsp.FrequencyDomainAdaptiveFilter' instead. |
| `DSPLTS` | 'dsp.LowerTriangularSolver' has been removed. With appropriate code changes, use 'mldivide' function or '\\' operator instead. |
| `DSPUTS` | 'dsp.UpperTriangularSolver' has been removed. With appropriate code changes, use 'mldivide' function or '\\' operator instead. |
| `DSPPMS` | 'dsp.PulseMetrics' has been removed. With appropriate code changes, use 'dutycycle', 'midcross', 'pulseperiod', 'pulsesep' or 'pulsewidth' instead. |
| `DSPTMS` | 'dsp.TransitionMetrics' has been removed. With appropriate code changes, use 'falltime', 'overshoot', 'risetime', 'settlingtime', 'slewrate' or 'undershoot' instead. |
| `FDESPULSESH` | 'fdesign.pulseshaping' has been removed. With appropriate code changes, use 'rcosdesign' or 'gaussdesign' instead. |
| `MCDECIM` | 'mfilt.cicdecim' will be removed in a future release. With appropriate code changes, use 'dsp.CICDecimator' instead. |
| `MCINTERP` | 'mfilt.cicinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| `MFARROW` | 'mfilt.farrowsrc' has been removed. With appropriate code changes, use 'dsp.FarrowRateConverter' instead. |
| `MFDECIM` | 'mfilt.firdecim' will be removed in a future release. With appropriate code changes, use 'dsp.FIRDecimator' instead. |
| `MFTDECIM` | 'mfilt.firtdecim' will be removed in a future release. With appropriate code changes, use 'dsp.FIRDecimator' instead. |
| `MFINTERP` | 'mfilt.firinterp' has been removed. With appropriate code changes, use 'dsp.FIRInterpolator' instead. |
| `MFSRC` | 'mfilt.firsrc' will be removed in a future release. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| `MFFTFINTERP` | 'mfilt.fftfirinterp' has been removed. With appropriate code changes, use 'dsp.FIRInterpolator' instead. |
| `MHINTERP` | 'mfilt.holdinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| `MIDECIM` | 'mfilt.iirdecim' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandDecimator' instead. |
| `MIINTERP` | 'mfilt.iirinterp' has been removed. With appropriate code changes, use 'dsp.IIRHalfbandInterpolator' instead. |
| `MIWDFDECIM` | 'mfilt.iirwdfdecim' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandDecimator' instead. |
| `MIWDFINTERP` | 'mfilt.iirwdfinterp' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandInterpolator' instead. |
| `MLINTERP` | 'mfilt.linearinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| `MFFDCM` | 'mfilt.firfracdecim' has been removed. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| `MFFINTRP` | 'mfilt.firfracinterp' has been removed. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| `DSPTS` | 'dsp.TimeScope' will be removed in a future release. Use 'timescope' instead, which is a direct replacement. |
| `DSPCEP2LPC` | 'dsp.CepstralToLPC' has been removed. There is no simple replacement for this. |
| `DSPLPC2AUTOCORR` | 'dsp.LPCToAutocorrelation' has been removed. With appropriate code changes, use 'poly2ac' instead. |
| `DSPLPC2CEP` | 'dsp.LPCToCepstral' has been removed. There is no simple replacement for this. |
| `DSPLPC2LSF` | 'dsp.LPCToLSF' has been removed. With appropriate code changes, use 'poly2lsf' instead. |
| `DSPLPC2RC` | 'dsp.LPCToRC' has been removed. With appropriate code changes, use 'poly2rc' instead. |
| `DSPLSF2LPC` | 'dsp.LSFToLPC' has been removed. With appropriate code changes, use 'lsf2poly' instead. |
| `DSPLSP2LPC` | 'dsp.LSPToLPC' has been removed. There is no simple replacement for this. |
| `DSPRC2AUTOCORR` | 'dsp.RCToAutocorrelation' has been removed. With appropriate code changes, use 'rc2ac' instead. |
| `DSPRC2LPC` | 'dsp.RCToLPC' has been removed. With appropriate code changes, use 'rc2poly' instead. |
| `DSPBURGEST` | 'dsp.BurgAREstimator' has been removed. With appropriate code changes, use 'arburg' instead. |
| `DSPBURGSPECEST` | 'dsp.BurgSpectrumEstimator' has been removed. With appropriate code changes, use 'pburg' instead. |
| `DSPDCT` | 'dsp.DCT' has been removed. With appropriate code changes, use 'dct' instead. |
| `DSPIDCT` | 'dsp.IDCT' has been removed. With appropriate code changes, use 'idct' instead. |
| `DSPPARAMEQ` | 'dsp.ParametricEQFilter' has been removed. With appropriate code changes, use 'designParamEQ' or 'MultibandParametricEQ' instead. |
| `DSPSCALARQUANTDEC` | 'dsp.ScalarQuantizerDecoder' has been removed. There is no simple replacement for this. |
| `DSPSCALARQUANTENC` | 'dsp.ScalarQuantizerEncoder' has been removed. There is no simple replacement for this. |
| `DSPUNIDEC` | 'dsp.UniformDecoder' has been removed. With appropriate code changes, use 'udecode' instead. |
| `DSPUNIENC` | 'dsp.UniformEncoder' has been removed. With appropriate code changes, use 'uencode' instead. |
| `DSPVECQUANTDEC` | 'dsp.VectorQuantizerDecoder' has been removed. There is no simple replacement for this. |
| `DSPVECQUANTENC` | 'dsp.VectorQuantizerEncoder' has been removed. There is no simple replacement for this. |
| `DSPSTATELVLS` | 'dsp.StateLevels' has been removed. With appropriate code changes, use 'statelevels' instead. |
| `FAFD` | 'farrow.fd' has been removed. With appropriate code changes, use 'dfilt.farrowfd' instead. |
| `FALFD` | 'farrow.linearfd' has been removed. With appropriate code changes, use 'dfilt.farrowlinearfd' instead. |
| `OPCDASUPP` | 'opc.daSupport' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCOPNOSF` | 'openosf' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCQID` | 'opcqid' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCQSTR` | 'opcqstr' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCQPRT` | 'opcqparts' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCDAQS` | 'opc.daQualityString' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `OPCDAEXPL` | 'opcDataAccessExplorer' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| `I2CFUN` | 'i2c' will be removed in a future release. With appropriate code changes, use 'device' method of 'ni845x' or 'aardvark' instead. |
| `IVIDC` | 'instrument.ivic.IviDCPwr' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVIDM` | 'instrument.ivic.IviDmm' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVIFG` | 'instrument.ivic.IviFgen' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVIPW` | 'instrument.ivic.IviPwrMeter' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVIRF` | 'instrument.ivic.IviRFSigGen' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVISP` | 'instrument.ivic.IviSpecAn' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVISW` | 'instrument.ivic.IviSwtch' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `IVISC` | 'instrument.ivic.IviScope' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| `INSTRR` | 'instrreset' will be removed in a future release. With appropriate code changes, instead use 'delete(serialportfind)', 'delete(tcpclientfind)', 'delete(tcpserverfind)', 'delete(udpportfind)', 'delete(visadevfind)', 'delete(aardvarkfind)', 'delete(ni845xfind)', or 'delete(icdevicefind)' when 'LegacyMode' for icdevice is false. |
| `INSTRH` | 'instrhelp' will be removed in a future release. Use 'help' instead, which is a direct replacement. |
| `INSTRF` | 'instrfind' will be removed in a future release. With appropriate code changes, instead use 'serialportfind', 'tcpclientfind', 'tcpserverfind', 'udpportfind', 'visadevfind', 'aardvarkfind', 'ni845xfind', or 'icdevicefind' when 'LegacyMode' for icdevice is false. |
| `INSTFA` | 'instrfindall' will be removed in a future release. With appropriate code changes, instead use 'serialportfind', 'tcpclientfind', 'tcpserverfind', 'udpportfind', 'visadevfind', 'aardvarkfind', 'ni845xfind', or 'icdevicefind' when 'LegacyMode' for icdevice is false. |
| `INSTCA` | 'instrcallback' will be removed in a future release. There is no simple replacement for this. |
| `INSTRN` | 'instrnotify' will be removed in a future release. There is no simple replacement for this. |
| `MKMID` | 'makemid' will be removed in a future release. There is no simple replacement for this. |
| `MIEIT` | 'midedit' will be removed in a future release. There is no simple replacement for this. |
| `MIDTT` | 'midtest' will be removed in a future release. There is no simple replacement for this. |
| `TMTL` | 'tmtool' will be removed in a future release. Use 'serialExplorer', 'tcpipExplorer', 'udpExplorer', 'visaExplorer', or 'instrumentExplorer' instead. |
| `IMJAVA` | 'im2java' has been removed. There is no simple replacement for this. |
| `ECF2LV` | 'ecef2lv' has been removed. With appropriate code changes, use 'ecef2enu' instead. |
| `LV2ECF` | 'lv2ecef' has been removed. With appropriate code changes, use 'enu2ecef' instead. |
| `RDFLDS` | 'readfields' has been removed. With appropriate code changes, use 'readmatrix', 'readtable', or a different file import function instead. |
| `RDMTX` | 'readmtx' has been removed. With appropriate code changes, use 'readmatrix', 'readtable', or a different file import function instead. |
| `RDFK5` | 'readfk5' has been removed. There is no simple replacement for this. |
| `SPCRD` | 'spcread' has been removed. With appropriate code changes, use 'readmatrix' instead. |
| `COLORM` | 'colorm' has been removed. There is no simple replacement for this. |
| `GTSEED` | 'getseeds' has been removed. There is no simple replacement for this. |
| `MKMAP` | 'makemapped' has been removed. There is no simple replacement for this. |
| `MOBJS` | 'mobjects' has been removed. There is no simple replacement for this. |
| `SEEDM` | 'seedm' has been removed. There is no simple replacement for this. |
| `LKBLNK` | 'leadblnk' has been removed. With appropriate code changes, use 'strtrim' instead. |
| `SFTSPC` | 'shiftspc' has been removed. With appropriate code changes, use 'strjust' instead. |
| `GTR2GLT` | 'geocentric2geodeticLat' has been removed. With appropriate code changes, use 'geodeticLatitudeFromGeocentric' instead. |
| `GLT2GTR` | 'geodetic2geocentricLat' has been removed. With appropriate code changes, use 'geocentricLatitude' instead. |
| `MEXTM` | 'extractm' has been removed. With appropriate code changes, use geospatial tables instead. |
| `QRYDT` | 'qrydata' has been removed. There is no simple replacement for this. |
| `CMBNT` | 'combntns' has been removed. Use 'nchoosek' instead, which is a direct replacement. |
| `FPSNM` | 'fipsname' has been removed. With appropriate code changes, use 'readgeotable' instead. |
| `GREPF` | 'grepfields' has been removed. With appropriate code changes, use 'textscan' instead. |
| `TRGLN` | 'tgrline' has been removed. With appropriate code changes, use 'readgeotable' instead. |
| `CLRUI` | 'colorui' has been removed. With appropriate code changes, use 'uisetcolor' instead. |
| `COMET` | 'cometm' has been removed. With appropriate code changes, use 'comet' instead. |
| `COMET3` | 'comet3m' has been removed. With appropriate code changes, use 'comet3' instead. |
| `MLYER` | 'mlayers' has been removed. There is no simple replacement for this. |
| `RSTCK` | 'restack' has been removed. With appropriate code changes, use 'uistack' instead. |
| `RTLYR` | 'rootlayr' has been removed. There is no simple replacement for this. |
| `EASTF` | 'eastof' has been removed. With appropriate code changes, use 'mod' instead. |
| `WESTF` | 'westof' has been removed. With appropriate code changes, use 'mod' instead. |
| `AT2GD` | 'aut2geod' has been removed. With appropriate code changes, use 'map.geodesy.AuthalicLatitudeConverter' instead. |
| `CN2GD` | 'cen2geod' has been removed. With appropriate code changes, use 'geodeticLatitudeFromGeocentric' instead. |
| `CF2GD` | 'cnf2geod' has been removed. With appropriate code changes, use 'map.geodesy.ConformalLatitudeConverter' instead. |
| `IS2GD` | 'iso2geod' has been removed. With appropriate code changes, use 'map.geodesy.IsometricLatitudeConverter' instead. |
| `PR2GD` | 'par2geod' has been removed. With appropriate code changes, use 'geodeticLatitudeFromParametric' instead. |
| `RC2GD` | 'rec2geod' has been removed. With appropriate code changes, use 'map.geodesy.RectifyingLatitudeConverter' instead. |
| `GD2AT` | 'geod2aut' has been removed. With appropriate code changes, use 'map.geodesy.AuthalicLatitudeConverter' instead. |
| `GD2CN` | 'geod2cen' has been removed. With appropriate code changes, use 'geocentricLatitude' instead. |
| `GD2CF` | 'geod2cnf' has been removed. With appropriate code changes, use 'map.geodesy.ConformalLatitudeConverter' instead. |
| `GD2IS` | 'geod2iso' has been removed. With appropriate code changes, use 'map.geodesy.IsometricLatitudeConverter' instead. |
| `GD2PR` | 'geod2par' has been removed. With appropriate code changes, use 'parametricLatitude' instead. |
| `GD2RC` | 'geod2rec' has been removed. With appropriate code changes, use 'map.geodesy.RectifyingLatitudeConverter' instead. |
| `DCWDT` | 'dcwdata' has been removed. With appropriate code changes, use 'vmap0data' instead. |
| `DCWGZ` | 'dcwgaz' has been removed. With appropriate code changes, use 'vmap0ui' instead. |
| `DCWRD` | 'dcwread' has been removed. With appropriate code changes, use 'vmap0read' instead. |
| `DCWHD` | 'dcwrhead' has been removed. With appropriate code changes, use 'vmap0rhead' instead. |
| `SYMBM` | 'symbolm' has been removed. With appropriate code changes, use 'scatterm' instead. |
| `TRACKUI` | 'trackui' has been removed. With appropriate code changes, use 'trackg' instead. |
| `SCIRCLUI` | 'scirclui' has been removed. With appropriate code changes, use 'scircleg' instead. |
| `ORIGINUI` | 'originui' has been removed. With appropriate code changes, use 'setm' instead. |
| `PARALLELUI` | 'parallelui' has been removed. With appropriate code changes, use 'setm' instead. |
| `SECTORG` | 'sectorg' has been removed. With appropriate code changes, use 'scircle1' instead. |
| `CLRMENU` | 'clrmenu' has been removed. With appropriate code changes, use 'colormapeditor' instead. |
| `MAPTRIM` | 'maptrim' has been removed. With appropriate code changes, use 'geocrop' or 'geoclip' instead. |
| `SURFDIST` | 'surfdist' has been removed. With appropriate code changes, use 'distance' instead. |
| `DEMDATAUI` | 'demdataui' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `VMAP0UI` | 'vmap0ui' has been removed. With appropriate code changes, use 'vmap0read' instead. |
| `MFWDT` | 'mfwdtran' has been removed. With appropriate code changes, use 'projfwd' instead. |
| `MINVT` | 'minvtran' has been removed. With appropriate code changes, use 'projinv' instead. |
| `SDTSINFO` | 'sdtsinfo' has been removed. With appropriate code changes, use 'georasterinfo' instead. |
| `LATLON2PIX` | 'latlon2pix' has been removed. With appropriate code changes, use 'geographicToIntrinsic' instead. |
| `LTLNV` | 'ltln2val' has been removed. With appropriate code changes, use 'geointerp' instead. |
| `MAP2PIX` | 'map2pix' has been removed. With appropriate code changes, use 'worldToIntrinsic' instead. |
| `MAPTM` | 'maptrims' has been removed. With appropriate code changes, use 'geocrop' instead. |
| `MESHGRAT` | 'meshgrat' has been removed. With appropriate code changes, use 'geographicGrid', 'linspace' or 'ndgrid' instead. |
| `NANM` | 'nanm' has been removed. With appropriate code changes, use 'nan' instead. |
| `ONEM` | 'onem' has been removed. With appropriate code changes, use 'ones' instead. |
| `PIX2LATLON` | 'pix2latlon' has been removed. With appropriate code changes, use 'intrinsicToGeographic' instead. |
| `PIX2MAP` | 'pix2map' has been removed. With appropriate code changes, use 'intrinsicToWorld' instead. |
| `PIXCENTERS` | 'pixcenters' has been removed. With appropriate code changes, use 'worldGrid' or 'geographicGrid' instead. |
| `RESZM` | 'resizem' has been removed. With appropriate code changes, use 'georesize' or 'imresize' instead. |
| `SETLTLN` | 'setltln' has been removed. With appropriate code changes, use 'intrinsicToGeographic' instead. |
| `SETPOSTN` | 'setpostn' has been removed. With appropriate code changes, use 'geographicToDiscrete' instead. |
| `SPZER` | 'spzerom' has been removed. With appropriate code changes, use 'sparse' instead. |
| `ZEROM` | 'zerom' has been removed. With appropriate code changes, use 'zeros' instead. |
| `MDTED` | 'dted' will be removed in a future release. With appropriate code changes, use 'readgeoraster' instead. |
| `ETOPO` | 'etopo' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `GLDEM` | 'globedem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `GTOPO` | 'gtopo30' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `SBATH` | 'satbath' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `SDTRD` | 'sdtsdemread' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `TBASE` | 'tbase' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `USGDM` | 'usgsdem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `USGKD` | 'usgs24kdem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| `DMCHN` | 'mimochan' has been removed. With appropriate code changes, use 'comm.MIMOChannel' instead. |
| `DPGUPDLG` | 'pagesetupdlg' has been removed. With appropriate code changes, use 'uiprintdlg' instead. |
| `MTHDPOOL` | 'parcluster.matlabpool' has been removed. With appropriate code changes, use 'parpool' instead. |
| `PDECT` | 'pdecont' has been removed. With appropriate code changes, use 'pdeplot' instead. |
| `PDESF` | 'pdesurf' has been removed. With appropriate code changes, use 'pdeplot' instead. |
| `POLYCPO` | 'polyspace.CodeProverOptions' has been removed. With appropriate code changes, use 'polyspace.Options' instead. |
| `POLYBFO` | 'polyspace.BugFinderOptions' has been removed. With appropriate code changes, use 'polyspace.Options' instead. |
| `PRPRE` | 'printpreview' will be removed in a future release. With appropriate code changes, use 'uiprintdlg' instead. |
| `PRDLG` | 'printdlg' will be removed in a future release. With appropriate code changes, use 'uiprintdlg' instead. |
| `EXPSE` | 'exportsetupdlg' will be removed in a future release. With appropriate code changes, use 'uiexportdlg' instead. |
| `RWA` | 'radarWaveformAnalyzer' will be removed in a future release. Use 'pulseWaveformAnalyzer' instead, which is a direct replacement. |
| `RCSFLT` | 'rcosflt' is unsupported and has been removed. With appropriate code changes, use 'rcosdesign' instead. |
| `RCSINE` | 'rcosine' is unsupported and has been removed. With appropriate code changes, use 'rcosdesign' instead. |
| `SVM2SUB` | 'Simulink.VariantManager.convertToVariant' will be removed in a future release. Use 'Simulink.VariantUtils.convertToVariantSubsystem' instead, which is a direct replacement. |
| `SVM2SUBA` | 'Simulink.VariantManager.convertToVariantAssemblySubsystem' will be removed in a future release. Use 'Simulink.VariantUtils.convertToVariantAssemblySubsystem' instead, which is a direct replacement. |
| `SVMVL` | 'Simulink.VariantManager.variantLegend' will be removed in a future release. Use 'Simulink.VariantUtils.variantLegend' instead, which is a direct replacement. |
| `SLLWARN` | 'sllastwarning' will be removed in a future release. With appropriate code changes, use 'lastwarn' instead. |
| `SLLERR1` | 'sllasterror' will be removed in a future release. Use an identifier on the CATCH block instead. |
| `SLLERR2` | 'sllastdiagnostic' will be removed in a future release. Use an identifier on the CATCH block instead. |
| `MOVIEVW` | 'movieview' has been removed. Use 'implay' instead, which is a direct replacement. |
| `IMAGEVW` | 'imageview' has been removed. Use 'imshow' instead, which is a direct replacement. |
| `SOUNDVW` | 'soundview' has been removed. With appropriate code changes, use 'audioread' with 'plot' or 'sound' instead. |
| `DWVRD` | 'wavread' has been removed. With appropriate code changes, use 'audioread' instead. |
| `DWVWR` | 'wavwrite' has been removed. With appropriate code changes, use 'audiowrite' instead. |
| `DWVFINF` | 'wavfinfo' has been removed. With appropriate code changes, use 'audioinfo' instead. |
| `WMCTR` | 'webmap' and its associated function 'wmcenter' will be removed in a future release. With appropriate code changes, use 'MapCenter' property of geographic axes object instead. |
| `WMCLS` | 'webmap' and its associated function 'wmclose' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'close' function instead. |
| `WMLMT` | 'webmap' and its associated function 'wmlimits' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geolimits' function instead. |
| `WMLIN` | 'webmap' and its associated function 'wmline' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geoplot' function instead. |
| `WMMKR` | 'webmap' and its associated function 'wmmarker' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geoiconchart' function instead. |
| `WMPYG` | 'webmap' and its associated function 'wmpolygon' will be removed in a future release. With appropriate code changes, use a geographic axes object, a 'geopolyshape' object, and 'geoplot' function instead. |
| `WMPNT` | 'webmap' and its associated function 'wmprint' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'exportgraphics' function instead. |
| `WMRMV` | 'webmap' and its associated function 'wmremove' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'delete' function instead. |
| `WMZOM` | 'webmap' and its associated function 'wmzoom' will be removed in a future release. With appropriate code changes, use 'ZoomLevel' property of geographic axes object instead. |
| `BETALIK1` | 'betalik1' has been removed. With appropriate code changes, use 'betalike' instead. |
| `SVMSMOSET` | 'svmsmoset' has been removed. With appropriate code changes, use 'fitcsvm' instead. |
| `LTEFS` | 'ltehdlFramesToSamples' has been removed. Use 'whdlFramesToSamples' instead, which is a direct replacement. |
| `LTESF` | 'ltehdlSamplesToFrames' has been removed. Use 'whdlSamplesToFrames' instead, which is a direct replacement. |
| `VHTLTFDEM` | 'wlanVHTLTFDemodulate' will be removed in a future release. With appropriate code changes, use 'wlanVHTDemodulate' instead. |
| `VHTDATAREC` | 'wlanVHTDataRecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTDataBitRecover' instead. |
| `VHTSIGAREC` | 'wlanVHTSIGARecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTSIGABitRecover' instead. |
| `VHTSIGBREC` | 'wlanVHTSIGBRecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTSIGBBitRecover' instead. |
| `BLBAF` | Manually setting 'BytesAvailableFcnCount' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| `BLTMT` | Manually setting 'Terminator' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'bluetooth' class to set value instead. |
| `BLIBS` | 'InputBufferSize' property of 'bluetooth' class will be removed in a future release. There is no simple replacement for this. |
| `SPPSS` | 'PinStatus' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'getpinstatus' method of 'serialport' class instead. |
| `SPTMT` | Manually setting 'Terminator' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'serialport' class to set value instead. |
| `SPIBS` | 'InputBufferSize' property of 'serialport' class will be removed in a future release. There is no simple replacement for this. |
| `TCTMT` | Manually setting 'Terminator' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'tcpclient' class to set value instead. |
| `TCIBS` | 'InputBufferSize' property of 'tcpclient' class will be removed in a future release. There is no simple replacement for this. |
| `TSTMT` | Manually setting 'Terminator' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'tcpserver' class to set value instead. |
| `TSIBS` | 'InputBufferSize' property of 'tcpserver' class will be removed in a future release. There is no simple replacement for this. |
| `UDTMT` | Manually setting 'Terminator' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'udpport' class to set value instead. |
| `UDIBS` | 'InputBufferSize' property of 'udpport' class will be removed in a future release. There is no simple replacement for this. |
| `VSTMT` | Manually setting 'Terminator' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'visadev' class to set value instead. |
| `VSIBS` | 'InputBufferSize' property of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| `VSBYA` | 'BytesAvailable' property of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| `VLSBC` | 'BackgroundColor' property has been removed. With appropriate code changes, use 'BackgroundColor' property of the parent 'Viewer3D' class instead. |
| `VLSCP` | 'CameraPosition' property has been removed. With appropriate code changes, use 'CameraPosition' property of the parent 'Viewer3D' class instead. |
| `VLSCT` | 'CameraTarget' property has been removed. With appropriate code changes, use 'CameraTarget' property of the parent 'Viewer3D' class instead. |
| `VLSCU` | 'CameraUpVector' property has been removed. With appropriate code changes, use 'CameraUpVector' property of the parent 'Viewer3D' class instead. |
| `VLSCV` | 'CameraViewAngle' property has been removed. There is no simple replacement for this. |
| `VLSIE` | 'InteractionsEnabled' property has been removed. With appropriate code changes, use 'Interactions' property of the parent 'Viewer3D' class instead. |
| `VLSLT` | 'Lighting' property has been removed. With appropriate code changes, use 'Lighting' property of the parent 'Viewer3D' class instead. |
| `VLSRD` | 'Renderer' property has been removed. With appropriate code changes, use 'RenderingStyle' property instead. |
| `VLSIC` | 'IsosurfaceColor' property has been removed. With appropriate code changes, use 'Colormap' property instead. |
| `VLSSF` | 'ScaleFactors' property has been removed. With appropriate code changes, use 'Transformation' property instead. |
| `VLSIV` | 'Isovalue' property has been removed. With appropriate code changes, use 'IsosurfaceValue' property instead. |
| `SESSIONQOD` | 'queueOutputData' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| `SESSIONACC` | 'addClockConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addclock' method of 'DataAcquisition' class instead. |
| `SESSIONATC` | 'addTriggerConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addtrigger' method of 'DataAcquisition' class instead. |
| `SESSIONISS` | 'inputSingleScan' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'read' method of 'DataAcquisition' class instead. |
| `SESSIONOSS` | 'outputSingleScan' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| `SESSIONSB` | 'startBackground' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'start' method of 'DataAcquisition' class instead. |
| `SESSIONSF` | 'startForeground' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| `BLFWR` | 'fwrite' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'write' method of 'bluetooth' class instead. |
| `BLSTR` | 'scanstr' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| `SPSTR` | 'scanstr' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| `TCSTR` | 'scanstr' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| `TSSTR` | 'scanstr' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| `UDSTR` | 'scanstr' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| `VSSTR` | 'scanstr' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| `SESSIONAIC` | 'addAnalogInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONADIC` | 'addAudioInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONAOC` | 'addAnalogOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONADOC` | 'addAudioOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONCOC` | 'addCounterOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONFGC` | 'addFunctionGeneratorChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONRC` | 'removeChannel' method of 'Session' class will be removed in a future release. Use 'removechannel' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONRSC` | 'resetCounters' method of 'Session' class will be removed in a future release. Use 'resetcounters' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONCIC` | 'addCounterInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| `SESSIONDIS` | 'DurationInSeconds' property of 'Session' class will be removed in a future release. With appropriate code changes, specify 'Duration' as argument to 'read' or 'start' instead. |
| `SESSIONNOS` | 'NumberOfScans' property of 'Session' class will be removed in a future release. With appropriate code changes, specify 'NumScans' as argument to 'read' or 'start' instead. |
| `SESSIONADC` | 'addDigitalChannel' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addinput' or 'addoutput' of 'DataAcquisition' class instead. |
| `SESSIONRCON` | 'removeConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'removeclock' or 'removetrigger' of 'DataAcquisition' class instead. |
| `SESSIONAL` | 'addlistener' method of 'Session' class will be removed in a future release. With appropriate code changes, use the DataAcquisition interface and its callback properties instead. |
| `PRINTRAS01` | '-dbmpmono' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS02` | '-dbmp' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS03` | '-dbmp16m' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS04` | '-dbmp256' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS05` | '-dhdf' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS06` | '-dpbm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS07` | '-dpbmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS08` | '-dpcxmono' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS09` | '-dpcx24b' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS10` | '-dpcx256' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS11` | '-dpcx16' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS12` | '-dpgm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS13` | '-dpgmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS14` | '-dppm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTRAS15` | '-dppmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| `PRINTPS1` | '-dps' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| `PRINTPS2` | '-dpsc' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| `PRINTPS3` | '-dps2' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| `PRINTPS4` | '-dpsc2' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| `SVFIGC` | 'compact' argument will be removed in a future release and currently has no effect. 'compact' behavior is always enabled. |
| `LEGMOP` | Syntax to call 'legend' with multiple outputs will be removed in a future release. There is no simple replacement for this. |
| `SMTHG` | 'GraphicsSmoothing' property will be removed in a future release. Graphics smoothing behavior has been enabled by default. Editing this property does not have an effect. |
| `SMTHGF` | 'DefaultFigureGraphicsSmoothing' setting will be removed in a future release. Graphics smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| `SMTHF` | 'FontSmoothing' property will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this property does not have an effect. |
| `SMTHFA` | 'DefaultAxesFontSmoothing' setting will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| `SMTHFT` | 'DefaultTextFontSmoothing' setting will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| `LINPROGD` | 'dual-simplex-legacy' algorithm for 'linprog' solver has been removed. With appropriate code changes, set 'Algorithm' value to 'dual-simplex-highs' or 'interior-point' instead. |
| `LINPROGA` | 'active-set' algorithm for 'linprog' solver has been removed. With appropriate code changes, set 'Algorithm' value to 'interior-point' or 'dual-simplex' instead. |
| `INTLLEG1` | 'Algorithm' option for the 'intlinprog' solver has been removed. The 'intlinprog' solver always uses the 'highs' algorithm. |
| `INTLLEG2` | 'BranchRule' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG3` | 'BranchingRule' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG4` | 'CutGeneration' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG5` | 'CutGenMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG6` | 'CutMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG7` | 'Heuristics' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG8` | 'HeuristicsMaxNodes' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG9` | 'IPPreprocess' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG10` | 'IntegerPreprocess' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG11` | 'TolInteger' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG12` | 'IntegerTolerance' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG13` | 'LPMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG14` | 'LPMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG15` | 'TolFunLP' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG16` | 'LPOptimalityTolerance' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG17` | 'NodeSelection' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG18` | 'RelObjThreshold' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG19` | 'ObjectiveImprovementThreshold' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG20` | 'RootLPAlgorithm' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG21` | 'RootLPMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLLEG22` | 'RootLPMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| `INTLPOUTA` | 'algorithm' field of the 'intlinprog' solution process summary has been removed. 'intlinprog' always uses the 'highs' algorithm. |
| `COEFFS` | The property 'FilterSpecification' has been removed. |
| `COEFF1` | The property 'FirstFilterCoefficients' has been removed. |
| `COEFF2` | The property 'SecondFilterCoefficients' has been removed. |
| `COEFF3` | The property 'ThirdFilterCoefficients' has been removed. |
| `COEFFD1` | The property 'FirstFilterCoefficientsDataType' has been removed. |
| `COEFFD2` | The property 'SecondFilterCoefficientsDataType' has been removed. |
| `COEFFD3` | The property 'ThirdFilterCoefficientsDataType' has been removed. |
| `COEFFC1` | The property 'CustomFirstFilterCoefficientsDataType' has been removed. |
| `COEFFC2` | The property 'CustomSecondFilterCoefficientsDataType' has been removed. |
| `COEFFC3` | The property 'CustomThirdFilterCoefficientsDataType' has been removed. |
| `READSZK` | The property 'KeyValueLimit' has been removed. |
| `READSZR` | The property 'RowsPerRead' has been removed. |
| `OPTMOPT` | solve(PROBLEM, OPTIONS) has been removed. Use solve(PROBLEM, 'Options', OPTIONS) instead. |
| `OPTMSLV` | solve(PROBLEM, SOLVER) has been removed. Use solve(PROBLEM, 'Solver', SOLVER) instead. |
| `OPTMNVP` | solve(PROBLEM, SOLVER, OPTIONS) has been removed. Use solve(PROBLEM, 'Solver', SOLVER, 'Options', OPTIONS) instead. |
| `POLYREP` | Use 'MergedReporting' instead of 'Reporting' for polyspace.Options. |
| `POLYCS` | Use 'MergedComputingSettings' instead of 'ComputingSettings' for polyspace.Options. |
| `ADTPATH` | 'path' will be removed in a future release. Use 'trajectory' instead. |
| `FPRENAME` | Input argument 'fixpoint' will be removed in a future release. Use 'fixedpoint' instead. |
| `XPCRENAME` | Input argument 'xpc' will be removed in a future release. Use 'slrealtime' instead. |
| `SLRTRENAME` | Input argument 'slrt' will be removed in a future release. Use 'slrealtime' instead. |
| `PSRENAME` | Input argument 'powersys' will be removed in a future release. Use 'sps' instead. |
| `DCRENAME` | Input argument 'distcomp' will be removed in a future release. Use 'parallel' instead. |
| `SERENAME` | Input argument 'simevents' will be removed in a future release. Use 'slde' instead. |
| `HHCNA` | Input argument 'North America' has been removed. Use 'hrn:here:data::olp-here-had:here-hdlm-protobuf-na-2' instead. |
| `HHCWE` | Input argument 'Western Europe' has been removed. Use 'hrn:here:data::olp-here-had:here-hdlm-protobuf-weu-2' instead. |
| `INSTHWB` | 'instrhwinfo('bluetooth',...)' will be removed in a future release. With appropriate code changes, use 'bluetoothlist' instead. |
| `INSTHWT` | 'instrhwinfo('tcpip')' will be removed in a future release. There is no simple replacement for this. |
| `INSTHWU` | 'instrhwinfo('udp')' will be removed in a future release. There is no simple replacement for this. |
| `CNNCGA` | 'cnncodegen' with 'targetlib' as 'arm-compute' has been removed. With appropriate code changes, use 'codegen' instead. |
| `CNNCGT` | 'cnncodegen' with 'targetlib' as 'tensorrt' has been removed. With appropriate code changes, use 'codegen' instead. |
| `CNNCGC` | 'cnncodegen' with 'targetlib' as 'cudnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| `CNNCGM` | 'cnncodegen' with 'targetlib' as 'mkldnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| `MAOUTL` | 'mapoutline' with referencing matrix has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAWFW` | 'worldfilewrite' with referencing matrix has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAARMT` | 'areamat' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAFDM` | 'findm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAPFIL` | 'mapprofile' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAGRDNT` | 'gradientm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MALOS2` | 'los2' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAVWSH` | 'viewshed' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MANORG` | 'neworig' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MACTRM` | 'contourm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MACTFM` | 'contourfm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MACT3M` | 'contour3m' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MASHM` | 'meshm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MASHL` | 'meshlsrm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAGTFW` | 'geotiffwrite' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAFLTM` | 'filterm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAVC2MX` | 'vec2mtx' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAIMBED` | 'imbedm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `MAG2I` | 'grid2image' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| `WFLRD` | 'worldfileread(worldFileName)' has been removed. With appropriate code changes, use 'worldfileread(worldFileName, coordinateSystemType, rasterSize)' instead. |
| `EGMGD` | 'egm96geoid(SAMPLEFACTOR,...)' has been removed. With appropriate code changes, use 'egm96geoid(R)' instead, where R is a geographic raster reference object. |
| `INSTHWS` | 'instrhwinfo('serial')' will be removed in a future release. With appropriate code changes, use 'serialportlist' instead. |
| `INSTHWSP` | 'instrhwinfo('serialport')' will be removed in a future release. With appropriate code changes, use 'serialportlist' instead. |
| `INSTHWV` | 'instrhwinfo('visa')' will be removed in a future release. With appropriate code changes, use 'visadevlist' instead. |
| `OPGLD` | 'opengl('data')' has been removed. With appropriate code changes, use 'rendererinfo' instead. |
| `OPGLO` | 'opengl' has been removed. There is no simple replacement for this. |
| `PMRTM1` | 'propagationModel('raytracing-image-method')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| `PMRTM2` | 'propagationModel('raytracing-imagemethod')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| `PMRTM3` | 'propagationModel('raytracingimage-method')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| `PMRTM4` | 'propagationModel('raytracingimagemethod')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method' , 'image')' syntax instead. |
| `INSTHWG` | 'instrhwinfo('gpib')' will be removed in a future release. With appropriate code changes, use 'visadevlist' instead. |
| `BLFOP` | 'fopen' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'bluetooth' constructor instead. |
| `SPFOP` | 'fopen' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'serialport' constructor instead. |
| `TCFOP` | 'fopen' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'tcpclient' constructor instead. |
| `TSFOP` | 'fopen' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'tcpserver' constructor instead. |
| `UDFOP` | 'fopen' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'udpport' constructor instead. |
| `VSFOP` | 'fopen' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visadev' constructor instead. |
| `SPFWR` | 'fwrite' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'write' method of 'serialport' class instead. |
| `TCFWR` | 'fwrite' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'write' method of 'tcpclient' class instead. |
| `TSFWR` | 'fwrite' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'write' method of 'tcpserver' class instead. |
| `UDFWR` | 'fwrite' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'write' method of 'udpport' class instead. |
| `VSFWR` | 'fwrite' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'write' method of 'visadev' class instead. |
| `BLFRD` | 'fread' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'read' method of 'bluetooth' class instead. |
| `SPFRD` | 'fread' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'read' method of 'serialport' class instead. |
| `TCFRD` | 'fread' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'read' method of 'tcpclient' class instead. |
| `TSFRD` | 'fread' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'read' method of 'tcpserver' class instead. |
| `UDFRD` | 'fread' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'read' method of 'udpport' class instead. |
| `VSFRD` | 'fread' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'read' method of 'visadev' class instead. |
| `BLFPR` | 'fprintf' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'bluetooth' class instead. |
| `SPFPR` | 'fprintf' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'serialport' class instead. |
| `TCFPR` | 'fprintf' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'tcpclient' class instead. |
| `TSFPR` | 'fprintf' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'tcpserver' class instead. |
| `UDFPR` | 'fprintf' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'udpport' class instead. |
| `VSFPR` | 'fprintf' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'visadev' class instead. |
| `BLFSF` | 'fscanf' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| `SPFSF` | 'fscanf' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| `TCFSF` | 'fscanf' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| `TSFSF` | 'fscanf' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| `UDFSF` | 'fscanf' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| `VSFSF` | 'fscanf' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| `BLFGL` | 'fgetl' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| `SPFGL` | 'fgetl' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| `TCFGL` | 'fgetl' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| `TSFGL` | 'fgetl' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| `UDFGL` | 'fgetl' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| `VSFGL` | 'fgetl' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| `BLFGT` | 'fgets' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| `SPFGT` | 'fgets' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| `TCFGT` | 'fgets' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| `TSFGT` | 'fgets' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| `UDFGT` | 'fgets' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| `VSFGT` | 'fgets' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| `BLFLI` | 'flushinput' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'bluetooth' class instead. |
| `SPFLI` | 'flushinput' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'serialport' class instead. |
| `TCFLI` | 'flushinput' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpclient' class instead. |
| `TSFLI` | 'flushinput' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpserver' class instead. |
| `UDFLI` | 'flushinput' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'udpport' class instead. |
| `VSFLI` | 'flushinput' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| `BLFCS` | 'fclose' method of 'bluetooth' class will be removed in a future release. There is no simple replacement for this. |
| `SPFCS` | 'fclose' method of 'serialport' class will be removed in a future release. There is no simple replacement for this. |
| `TCFCS` | 'fclose' method of 'tcpclient' class will be removed in a future release. There is no simple replacement for this. |
| `TSFCS` | 'fclose' method of 'tcpserver' class will be removed in a future release. There is no simple replacement for this. |
| `UDFCS` | 'fclose' method of 'udpport' class will be removed in a future release. There is no simple replacement for this. |
| `VSFCS` | 'fclose' method of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| `SPBBW` | 'binblockwrite' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'serialport' class instead. |
| `TCBBW` | 'binblockwrite' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'tcpclient' class instead. |
| `TSBBW` | 'binblockwrite' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'tcpserver' class instead. |
| `VSBBW` | 'binblockwrite' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'visadev' class instead. |
| `SPBBR` | 'binblockread' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'serialport' class instead. |
| `TCBBR` | 'binblockread' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'tcpclient' class instead. |
| `TSBBR` | 'binblockread' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'tcpserver' class instead. |
| `VSBBR` | 'binblockread' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'visadev' class instead. |
| `TCQRY` | 'query' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writeread' method of 'tcpclient' class instead. |
| `VSQRY` | 'query' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writeread' method of 'visadev' class instead. |
| `VSCRD` | 'clrdevice' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| `VSSPL` | 'spoll' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visastatus' method of 'visadev' class instead. |
| `VSTGR` | 'trigger' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visatrigger' method of 'visadev' class instead. |
| `SPBAF` | Manually setting 'BytesAvailableFcnCount' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| `TCBAF` | Manually setting 'BytesAvailableFcnCount' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| `TSBAF` | Manually setting 'BytesAvailableFcnCount' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| `UDBAF` | Manually setting 'BytesAvailableFcnCount' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| `BLBAN` | Manually setting 'BytesAvailableFcn' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| `SPBAN` | Manually setting 'BytesAvailableFcn' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| `TCBAN` | Manually setting 'BytesAvailableFcn' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| `TSBAN` | Manually setting 'BytesAvailableFcn' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| `UDBAN` | Manually setting 'BytesAvailableFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| `BLBAM` | Manually setting 'BytesAvailableFcnMode' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| `SPBAM` | Manually setting 'BytesAvailableFcnMode' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| `TCBAM` | Manually setting 'BytesAvailableFcnMode' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| `TSBAM` | Manually setting 'BytesAvailableFcnMode' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| `UDBAM` | Manually setting 'BytesAvailableFcnMode' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| `BLBAB` | 'BytesAvailable' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'bluetooth' class instead. |
| `SPBAB` | 'BytesAvailable' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'serialport' class instead. |
| `TCBAB` | 'BytesAvailable' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'tcpclient' class instead. |
| `TSBAB` | 'BytesAvailable' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'tcpserver' class instead. |
| `UDBAB` | 'BytesAvailable' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'udpport' class instead. |
| `BLEFN` | 'ErrorFcn' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'bluetooth' class instead. |
| `SPEFN` | 'ErrorFcn' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'serialport' class instead. |
| `TCEFN` | 'ErrorFcn' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'tcpclient' class instead. |
| `TSEFN` | 'ErrorFcn' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'tcpserver' class instead. |
| `UDEFN` | 'ErrorFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'udpport' class instead. |
| `VSEFN` | 'ErrorFcn' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'visadev' class instead. |
| `BLREN` | 'RemoteName' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'Name' property of 'bluetooth' class instead. |
| `BLRID` | 'RemoteID' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'Address' property of 'bluetooth' class instead. |
| `VSPSS` | 'PinStatus' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'getpinstatus' method of 'visadev' class instead. |
| `SPDTR` | 'DataTerminalReady' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'setDTR' method of 'serialport' class instead. |
| `VSDTR` | 'DataTerminalReady' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'setDTR' method of 'visadev' class instead. |
| `SPRTS` | 'RequestToSend' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'setRTS' method of 'serialport' class instead. |
| `VSRTS` | 'RequestToSend' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'setRTS' method of 'visadev' class instead. |
| `TCNTR` | 'NetworkRole' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'tcpclient' constructor instead. |
| `TSNTR` | 'NetworkRole' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'tcpserver' constructor instead. |
| `TCTDY` | 'TransferDelay' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'EnableTransferDelay' property of 'tcpclient' class instead. |
| `TCRPT` | 'RemotePort' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'Port' property of 'tcpclient' class instead. |
| `TSRPT` | 'RemotePort' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerPort' property of 'tcpserver' class instead. |
| `TCRHT` | 'RemoteHost' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'Address' property of 'tcpclient' class instead. |
| `TSRHT` | 'RemoteHost' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ClientAddress' property of 'tcpserver' class instead. |
| `TSLHT` | 'LocalHost' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| `TSLPM` | 'LocalPortMode' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| `TSLPT` | 'LocalPort' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| `UDDTM` | 'DatagramTerminateMode' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'udpport' constructor instead. |
| `UDODP` | 'OutputDatagramPacketSize' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'OutputDatagramSize' property of 'udpport' class instead. |
| `VSEMD` | 'EOSMode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method and 'EOIMode' property of 'visadev' class instead. |
| `VSECC` | 'EOSCharCode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'visadev' class instead. |
| `VSMID` | 'ManufacturerID' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'VendorID' property of 'visadev' class instead. |
| `VSMLC` | 'ModelCode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ProductID' property of 'visadev' class instead. |
| `BLFLO` | 'flushoutput' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'bluetooth' class instead. |
| `SPFLO` | 'flushoutput' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'serialport' class instead. |
| `TCFLO` | 'flushoutput' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpclient' class instead. |
| `TSFLO` | 'flushoutput' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpserver' class instead. |
| `UDFLO` | 'flushoutput' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'udpport' class instead. |
| `VSFLO` | 'flushoutput' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| `UDDRF` | 'DatagramReceivedFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class instead. |
| `VSRSN` | 'RsrcName' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ResourceName' property of 'visadev' class instead. |
| `ELEVT` | 'elevation' has been removed. With appropriate code changes, use 'geodetic2aer' instead. |
| `INSTHWI2C` | 'instrhwinfo('i2c')' will be removed in a future release. With appropriate code changes, use 'ni845xlist' or 'aardvarklist' instead. |

### Forward Compatibility (7 checks)

| Check ID | Message |
| -------- | ------- |
| `FCLEN` | Identifiers longer than 63 characters are not supported before R2025a. |
| `FCCPV` | Class property validation is not available before R2017a. |
| `FCDQS` | Double-quoted strings are not available before R2017a. Use character vectors for scalar strings or cell arrays of character vectors for string arrays. |
| `FCFAV` | Function argument validation is not available before R2019b. |
| `FCHBL` | Hexadecimal and binary literals are not available before R2019b. Use 'hex2dec' and 'bin2dec' instead. |
| `FCLFS` | Local functions in a script are not available before R2016b. |
| `FCNVA` | Name=Value syntax is not available before R2021a. Use comma-separated syntax instead. |

### Behavior Changes (905 checks)

| Check ID | Message |
| -------- | ------- |
| `SHVAI` | Explicitly define shared variables in the parent function before calling the nested function. MATLAB does not share uninitialized variables between a nested function and the parent function. |
| `LENEMP` | Passing in text with no characters will omit the object from appearing in the legend. To revert to the old behavior, use a whitespace character instead of text with no characters. |
| `INTRPC` | 'interp1(...,'cubic')' changed in R2020b to perform cubic convolution. To continue using shape-preserving piecewise cubic interpolation, use 'interp1(...,'pchip')' instead. |
| `IDISVARHIGH` | Variable must be explicitly defined before first use. In some cases, the definition was not required in previous releases, but it is now required. |
| `LEGPVPAIR` | 'legend' has changed and might interpret the name of an argument as a legend property instead of a label. |
| `CLBGEN` | Starting in R2020a, interfaces created by 'clibgen.generateLibraryDefinition' return clib.array object instead of the equivalent MATLAB array for primitive types. |
| `CLBBLD` | Starting in R2020a, interfaces created by 'clibgen.buildInterface' return clib.array object instead of the equivalent MATLAB array for primitive types. |
| `COLMP` | In R2019a and previous releases, the default colormap size is 64. Starting in R2019b, colormaps have 256 colors by default. |
| `FDTAG` | 'findall' with 'Exploration.Pan', 'Exploration.ZoomIn', etc. might return empty because the data exploration buttons have moved from the figure toolbar to the axes toolbar. |
| `NSTIMP` | Nested functions now inherit import statements from this parent function. |
| `IDISVARLOW` | To avoid a potential conflict with functions on the path, explicitly define the variable before indexing into it. |
| `WEBBEHAVE` | The 'web' function now opens external sites in your system browser by default. |
| `GLGRI` | Starting R2021a, the second output of 'geoloc2grid' is a geographic raster reference object instead of a referencing vector. |
| `V2MTX` | Starting R2021a, the second output of 'vec2mtx' is a geographic raster reference object instead of a referencing vector. |
| `PTCLO` | Changing the axes LineStyleOrder or ColorOrder properties of an existing chart now affects the chart immediately. |
| `PTDLO` | Specifying multiple line styles in the axes LineStyleOrder might result in charts that render differently than in the previous releases. |
| `JAPIEXT1` | 'com.teamdev.jxbrowser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT4` | 'javax.security.auth' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT5` | 'javax.transaction.xa' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT7` | 'org.apache.el' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT8` | 'org.apache.juli' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT9` | 'org.apache.tomcat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT10` | 'org.apache.xmlrpc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT11` | 'org.jboss.netty' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT14` | 'org.ros.actionlib' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT15` | 'org.ros.address' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT16` | 'org.ros.concurrent' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT17` | 'org.ros.exception' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT18` | 'org.ros.gradle_plugins' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT19` | 'org.ros.internal' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT21` | 'org.ros.master' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT22` | 'org.ros.message' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT23` | 'org.ros.namespace' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT24` | 'org.ros.node' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT31` | 'org.apache.commons.codec' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT35` | 'org.apache.commons.io' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT36` | 'org.apache.commons.lang' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT38` | 'org.apache.commons.logging' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT39` | 'org.apache.commons.math3' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT40` | 'org.apache.commons.net' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT41` | 'org.apache.http' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT42` | 'org.apache.log4j' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT43` | 'org.apache.xerces' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `CLBARY` | Starting in R2020a, clib.array object is the default return value, instead of the equivalent MATLAB array for primitive types. Notify your user to update code to use clib arrays. To revert to the old behavior, call 'clibgen.generateLibraryDefinition' or 'clibgen.buildInterface' with the 'ReturnCArrays' argument set to false. |
| `JAPIEXT20` | 'org.ros.master' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT25` | 'org.ros.rosjava_geometry' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT26` | 'org.ros.tf2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT27` | 'org.ros.time' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT28` | 'org.xbill.DNS' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT29` | 'org.jmol.quantum' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT32` | 'ice.pilots.notsupported' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT33` | 'ice.pilots.mathml' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT34` | 'com.drew.metadata' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT37` | 'ice.pilots.domviewer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT44` | 'cryptix.provider.mode' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT47` | 'ice.pilots.pdf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT50` | 'org.dom4j.swing' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT54` | 'opennlp.tools.dictionary' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT55` | 'ice.util.alg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT56` | 'org.jmol.multitouch' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT62` | 'org.jmol.minimize' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT63` | 'ice.util.awt' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT64` | 'schemaorg_apache_xmlbeans.system.s8C3F193EE11A2F798ACF65489B9E6078' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT65` | 'opennlp.tools.stemmer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT66` | 'opennlp.tools.ngram' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT67` | 'org.jsoup.select' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT74` | 'org.drizzle.jdbc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT75` | 'org.jmol.bspt' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT80` | 'ice.pilots.jmf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT82` | 'thredds.inventory.partition' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT84` | 'de.l3s.boilerpipe' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT85` | 'ice.pilots.es' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT88` | 'org.bouncycastle.pkix' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT90` | 'org.dom4j.xpath' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT91` | 'ice.pilots.text' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT92` | 'thredds.cataloggen.datasetenhancer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT96` | 'ice.util.memory' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT97` | 'org.jmol.translation' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT98` | 'opennlp.tools.cmdline' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT103` | 'ice.scripters.js' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT109` | 'org.bouncycastle.i18n' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT111` | 'org.bouncycastle.operator' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT112` | 'schemaorg_apache_xmlbeans.system.sXMLSCHEMA' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT114` | 'opennlp.tools.chunker' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT115` | 'org.jsoup.safety' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT116` | 'org.bouncycastle.cert' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT118` | 'org.jmol.shapespecial' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT120` | 'thredds.catalog2.xml' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT121` | 'thredds.catalog.dl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT123` | 'net.jcip.annotations' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT126` | 'org.jmol.adapter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT134` | 'opennlp.tools.formats' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT135` | 'com.mchange.v1' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT136` | 'com.lowagie.tools' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT138` | 'org.dom4j.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT145` | 'org.apache.jempbox' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT160` | 'ice.net.socks' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT169` | 'org.bouncycastle.x509' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT170` | 'org.jsoup.examples' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT173` | 'org.apache.mina' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT178` | 'com.mchange.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT181` | 'thredds.catalog2.builder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT187` | 'com.drew.tools' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT192` | 'ice.util.security' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT195` | 'org.apache.james' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT196` | 'org.jmol.export' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT197` | 'org.jmol.symmetry' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT198` | 'org.mozilla.universalchardet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT206` | 'opennlp.tools.coref' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT207` | 'opennlp.tools.postag' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT209` | 'org.mozilla.javascript' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT211` | 'ice.dom.css' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT212` | 'org.mozilla.classfile' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT218` | 'org.dom4j.datatype' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT219` | 'ice.util.unit' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT230` | 'com.drew.imaging' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT231` | 'jj2000.j2k.codestream' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT232` | 'org.jmol.popup' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT235` | 'org.apache.tika' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT240` | 'opennlp.tools.tokenize' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT242` | 'javolution.util.stripped' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT259` | 'jj2000.j2k.encoder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT266` | 'org.bouncycastle.tsp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT267` | 'com.cybozu.labs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT269` | 'org.jmol.smiles' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT272` | 'com.sparshui.inputdevice' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT274` | 'ice.storm.print' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT275` | 'org.jmol.api' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT277` | 'org.dom4j.jaxb' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT281` | 'org.jmol.console' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT294` | 'org.apache.ftpserver' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT301` | 'thredds.catalog2.simpleImpl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT305` | 'org.jmol.atomdata' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT308` | 'com.almworks.sqlite4java' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT309` | 'org.bouncycastle.crypto' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT312` | 'thredds.catalog2.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT315` | 'com.rometools.utils' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT316` | 'ice.util.encoding' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT317` | 'com.mchange.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT320` | 'ice.net.pac' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT321` | 'cryptix.util.core' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT324` | 'schemaorg_apache_xmlbeans.system.sXMLLANG' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT326` | 'thredds.catalog.crawl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT327` | 'thredds.catalog.query' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT330` | 'org.openscience.jmol' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT331` | 'ice.dom.html' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT336` | 'ice.pilots.applet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT340` | 'org.jmol.modelkit' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT343` | 'jj2000.j2k.wavelet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT348` | 'org.bouncycastle.cms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT352` | 'org.bouncycastle.jce' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT353` | 'ice.net.mailto' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT358` | 'jj2000.j2k.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT360` | 'jj2000.j2k.quantization' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT364` | 'com.coremedia.iso' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT369` | 'thredds.cataloggen.catalogrefexpander' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT378` | 'org.jmol.script' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT380` | 'com.optimaize.langdetect' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT381` | 'net.arnx.jsonic' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT390` | 'schemaorg_apache_xmlbeans.system.sF1327CCA741569E70F9CA8C9AF9B44B2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT394` | 'xjava.security.interfaces' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT398` | 'org.dom4j.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT401` | 'org.bouncycastle.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT403` | 'org.bouncycastle.pkcs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT405` | 'org.bouncycastle.dvcs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT408` | 'se.fishtank.css' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT409` | 'ice.net.doc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT411` | 'com.adobe.xmp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT416` | 'opennlp.tools.namefind' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT420` | 'opennlp.tools.doccat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT421` | 'com.sun.java' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT426` | 'thredds.cataloggen.config' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT428` | 'org.bouncycastle.mozilla' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT430` | 'opennlp.tools.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT442` | 'jj2000.j2k.roi' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT445` | 'org.bouncycastle.math' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT447` | 'org.dom4j.dom' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT449` | 'jj2000.j2k.entropy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT451` | 'org.bouncycastle.eac' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT455` | 'org.jmol.i18n' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT459` | 'thredds.inventory.filter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT466` | 'opennlp.maxent.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT467` | 'net.didion.jwnl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT470` | 'cryptix.provider.rsa' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT474` | 'org.jmol.shapebio' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT477` | 'ice.pilots.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT485` | 'jj2000.j2k.fileformat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT486` | 'org.bouncycastle.mail' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT487` | 'opennlp.tools.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT491` | 'org.jmol.g3d' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT495` | 'cryptix.provider.cipher' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT502` | 'ice.util.net' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT503` | 'jj2000.j2k.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT505` | 'ice.util.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT508` | 'org.jmol.modelset' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT510` | 'com.mchange.v2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT511` | 'org.dom4j.rule' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT513` | 'org.bouncycastle.pqc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT516` | 'org.jmol.modelsetbio' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT517` | 'be.frma.langguess' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT520` | 'schemaorg_apache_xmlbeans.system.sXMLTOOLS' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT521` | 'cryptix.provider.key' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT523` | 'thredds.crawlabledataset.filter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT533` | 'thredds.cataloggen.inserter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT534` | 'org.codehaus.stax2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT535` | 'jj2000.j2k.decoder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT536` | 'org.dom4j.bean' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT540` | 'org.bouncycastle.openssl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT547` | 'ice.net.proxy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT548` | 'org.dom4j.tree' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT553` | 'com.lowagie.bc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT557` | 'uk.ac.rdg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT563` | 'org.apache.sis' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT564` | 'org.dom4j.xpp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT571` | 'com.sparshui.common' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT589` | 'Acme.JPM.Encoders' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT590` | 'org.json.zip' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT592` | 'org.bouncycastle.jcajce' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT595` | 'com.sparshui.server' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT596` | 'org.jmol.shapesurface' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT597` | 'org.bouncycastle.asn1' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT598` | 'opennlp.tools.sentdetect' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT601` | 'com.rometools.rome' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT603` | 'com.drew.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT604` | 'thredds.crawlabledataset.sorter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT605` | 'ice.util.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT610` | 'com.sparshui.client' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT611` | 'thredds.catalog.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT614` | 'org.bouncycastle.voms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT616` | 'com.lowagie.text' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT618` | 'org.jsoup.helper' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT622` | 'org.jmol.shape' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT627` | 'ice.util.swing' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT628` | 'org.jmol.jvxl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT635` | 'org.cometd.client' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT637` | 'ice.pilots.pdfgo' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT638` | 'org.json.simple' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT639` | 'org.jsoup.nodes' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT643` | 'ice.pilots.svg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT644` | 'thredds.catalog.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT647` | 'org.ccil.cowan' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT653` | 'org.jmol.geodesic' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT655` | 'jj2000.j2k.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT661` | 'org.dom4j.dtd' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT670` | 'cryptix.provider.md' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT671` | 'opennlp.tools.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT672` | 'com.sparshui.gestures' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT680` | 'org.jmol.viewer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT683` | 'ice.pilots.html4' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT685` | 'schemaorg_apache_xmlbeans.system.sXMLCONFIG' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT686` | 'opennlp.maxent.quasinewton' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT688` | 'org.jsoup.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT689` | 'com.ctc.wstx' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT696` | 'org.jmol.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT697` | 'org.itadaki.bzip2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT699` | 'com.codahale.metrics' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT700` | 'com.datastax.driver' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT713` | 'com.terracotta.entity' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT716` | 'io.netty.bootstrap' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT717` | 'io.netty.buffer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT718` | 'io.netty.channel' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT719` | 'io.netty.handler' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT720` | 'io.netty.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT735` | 'net.sf.ehcache' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT736` | 'org.apache.directory' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT745` | 'org.joda.time' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT781` | 'org.springframework.jms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT787` | 'org.springframework.messaging' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT791` | 'org.springframework.oxm' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT809` | 'org.terracotta.context' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT810` | 'org.terracotta.modules' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT811` | 'org.terracotta.statistics' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT813` | 'org.xerial.snappy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT814` | 'javax.help.event' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT815` | 'javax.help.plaf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT816` | 'javax.help.resources' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT817` | 'javax.help.search' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT818` | 'javax.help.tagext' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT824` | 'schemaorg_apache_xmlbeans.system.sD023D6490046BA0250A839A9AD24C443' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `ROSDFOBJECT` | For improved performance and code generation workflows, specify 'DataFormat' name-value argument as 'struct'. In a future release, the default format will change to 'struct'. |
| `ROSSCFUN` | Add the service type as the second input argument to 'rossvcclient'. In a future release, the service type will be a required input argument. |
| `ROSSCPKG` | Add the service type as the third input argument to 'ros.ServiceClient'. In a future release, the service type will be a required input argument. |
| `ROSDFMISSING` | Use name-value argument 'DataFormat' to specify the message format as the default message format will change to 'struct' in a future release. |
| `JAPIEXT2` | 'javax.annotation.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT3` | 'javax.annotation.sql' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT6` | 'javax.websocket.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT12` | 'org.jdesktop.layout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT13` | 'org.jdesktop.swingx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT30` | 'com.jidesoft.icons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT45` | 'info.clearthought.layout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT46` | 'org.antlr.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT48` | 'org.powermock.configuration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT49` | 'javax.mail.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT51` | 'com.thaiopensource.validate' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT52` | 'org.opengis.webservice' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT53` | 'com.google.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT57` | 'freemarker.ext.jython' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT58` | 'org.h2.store' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT59` | 'com.google.protobuf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT60` | 'org.h2.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT61` | 'org.jaxen.expr' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT68` | 'org.antlr.codegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT69` | 'org.mockito.listeners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT70` | 'org.h2.result' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT71` | 'com.jogamp.newt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT72` | 'org.mockito.mock' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT73` | 'org.opengis.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT76` | 'org.h2.constraint' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT77` | 'org.powermock.tests' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT78` | 'org.h2.tools' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT79` | 'javassist.bytecode.annotation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT81` | 'org.h2.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT83` | 'org.mockito.quality' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT86` | 'javax.servlet.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT87` | 'org.geotools.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT89` | 'org.mockito.hamcrest' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT93` | 'javax.mail.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT94` | 'com.jgoodies.looks' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT95` | 'com.graphbuilder.math' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT99` | 'mwhtmlguitest.org.apache' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT100` | 'javax.mail.search' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT101` | 'net.sf.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT102` | 'org.powermock.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT104` | 'org.geotools.resources' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT105` | 'org.openxml4j.samples' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT106` | 'org.w3c.css' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT107` | 'org.cef.handler' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT108` | 'org.mortbay.jetty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT110` | 'jogamp.nativewindow.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT113` | 'com.jidesoft.popup' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT117` | 'org.apache.batik' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT119` | 'net.jpountz.xxhash' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT122` | 'org.apache.axis2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT124` | 'com.jogamp.opengl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT125` | 'com.reuters.sdist' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT127` | 'org.jaxen.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT128` | 'org.aopalliance.intercept' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT129` | 'org.jaxen.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT130` | 'jogamp.opengl.gl4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT131` | 'com.thaiopensource.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT132` | 'com.jidesoft.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT133` | 'com.graphbuilder.curve' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT137` | 'org.jacoco.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT139` | 'org.objenesis.instantiator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT140` | 'net.jini.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT141` | 'edu.uci.ics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT142` | 'org.geotools.parameter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT143` | 'com.googlecode.javaewah32' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT144` | 'org.jaxen.function' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT146` | 'com.jidesoft.spinner' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT147` | 'org.mockito.exceptions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT148` | 'org.objectweb.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT149` | 'org.jdom2.located' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT150` | 'org.mortbay.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT151` | 'antlr.debug.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT152` | 'javax.servlet.annotation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT153` | 'net.jini.space' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT154` | 'org.apache.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT155` | 'org.jdom2.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT156` | 'org.antlr.stringtemplate' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT157` | 'com.graphbuilder.geom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT158` | 'antlr.actions.python' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT159` | 'org.eclipse.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT161` | 'org.w3c.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT162` | 'javassist.tools.reflect' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT163` | 'com.jidesoft.range' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT164` | 'org.jdom.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT165` | 'org.mockito.stubbing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT166` | 'org.iso_relax.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT167` | 'org.eclipse.xtend2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT168` | 'jogamp.newt.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT171` | 'jogamp.graph.font' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT172` | 'org.jdom2.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT174` | 'org.jaxen.xom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT175` | 'org.geotools.factory' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT176` | 'javax.xml.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT177` | 'net.jini.lookup' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT179` | 'org.eclipse.osgi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT180` | 'abbot.editor.recorder' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT182` | 'org.h2.mvstore' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT183` | 'org.mortbay.resource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT184` | 'jogamp.opengl.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT185` | 'jogamp.opengl.x11' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT186` | 'org.tanukisoftware.wrapper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT188` | 'org.geotools.map' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT189` | 'org.cyberneko.html' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT190` | 'org.openxmlformats.schemas' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT191` | 'com.reuters.rmtes' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT193` | 'net.bytebuddy.utility' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT194` | 'freemarker.ext.jsp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT199` | 'org.mockito.plugins' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT200` | 'org.mockito.verification' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT201` | 'com.sun.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT202` | 'jogamp.common.jvm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT203` | 'org.geotools.gml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT204` | 'org.hamcrest.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT205` | 'freemarker.ext.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT208` | 'net.jini.discovery' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT210` | 'org.h2.jmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT213` | 'net.jini.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT214` | 'antlr.actions.csharp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT215` | 'javassist.bytecode.analysis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT216` | 'javax.xml.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT217` | 'abbot.editor.actions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT220` | 'org.jacoco.report' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT221` | 'org.powermock.reflect' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT222` | 'jogamp.opengl.macosx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT223` | 'org.apache.xmlcommons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT224` | 'org.jaxen.javabean' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT225` | 'net.sf.xslthl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT226` | 'org.mockito.codegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT227` | 'net.jini.activation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT228` | 'net.bytebuddy.matcher' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT229` | 'net.bytebuddy.dynamic' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT233` | 'org.mockito.invocation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT234` | 'org.hamcrest.number' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT236` | 'org.openxml4j.document' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT237` | 'freemarker.ext.jdom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT238` | 'org.geotools.feature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT239` | 'org.eclipse.jgit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT241` | 'org.h2.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT243` | 'net.jini.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT244` | 'com.jidesoft.csv' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT245` | 'org.tmatesoft.svn' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT246` | 'freemarker.ext.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT247` | 'org.antlr.grammar' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT248` | 'com.jidesoft.field' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT249` | 'jogamp.nativewindow.windows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT250` | 'jogamp.nativewindow.macosx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT251` | 'org.geotools.coverage' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT252` | 'org.h2.bnf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT253` | 'org.h2.jdbcx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT254` | 'org.mockito.session' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT255` | 'org.powermock.mockpolicies' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT256` | 'jogamp.common.os' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT257` | 'org.apache.fontbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT258` | 'net.jini.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT260` | 'org.apache.taglibs' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT261` | 'org.jacoco.agent' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT262` | 'freemarker.ext.servlet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT263` | 'jogamp.opengl.gl2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT264` | 'com.thaiopensource.relaxng' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT265` | 'org.h2.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT268` | 'org.geotools.measure' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT270` | 'org.cometd.websocket' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT271` | 'org.jdom2.input' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT273` | 'abbot.editor.editors' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT276` | 'org.eclipse.xtext' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT278` | 'org.etsi.uri' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT279` | 'org.opengis.spatialschema' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT280` | 'org.opengis.feature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT282` | 'com.vividsolutions.jts' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT283` | 'org.apache.ws' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT284` | 'com.intel.bluetooth' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT285` | 'com.jidesoft.alert' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT286` | 'org.iso_relax.dispatcher' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT287` | 'org.antlr.gunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT288` | 'jogamp.newt.swt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT289` | 'com.jidesoft.margin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT290` | 'org.geotools.data' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT291` | 'org.cef.browser' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT292` | 'com.jogamp.graph' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT293` | 'org.geotools.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT295` | 'org.apache.jasper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT296` | 'com.thoughtworks.xstream' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT297` | 'org.apache.commons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT298` | 'org.h2.command' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT299` | 'org.geotools.geometry' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT300` | 'com.vladium.jcd' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT302` | 'org.junit.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT303` | 'org.powermock.api' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT304` | 'net.sf.saxon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT306` | 'com.bloomberglp.blpapi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT307` | 'com.reuters.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT310` | 'freemarker.debug.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT311` | 'javax.servlet.descriptor' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT313` | 'com.sun.midp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT314` | 'com.jidesoft.utils' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT318` | 'javax.mail.internet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT319` | 'abbot.script.parsers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT322` | 'com.vladium.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT323` | 'freemarker.ext.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT325` | 'org.aopalliance.aop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT328` | 'org.h2.message' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT329` | 'com.jogamp.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT332` | 'javax.xml.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT333` | 'org.eclipse.jdt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT334` | 'org.mortbay.start' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT335` | 'javax.wsdl.extensions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT337` | 'org.opengis.metadata' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT338` | 'org.slf4j.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT339` | 'org.h2.index' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT341` | 'com.jidesoft.jdk' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT342` | 'com.jidesoft.navigation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT344` | 'freemarker.ext.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT345` | 'org.eclipse.e4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT346` | 'org.junit.runner' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT347` | 'org.apache.xmlgraphics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT349` | 'com.jidesoft.pane' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT350` | 'org.hamcrest.generator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT351` | 'org.iso_relax.verifier' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT354` | 'org.opengis.coverage' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT355` | 'org.antlr.tool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT356` | 'org.mortbay.thread' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT357` | 'org.mortbay.naming' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT359` | 'com.jidesoft.grouper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT361` | 'jogamp.opengl.windows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT362` | 'org.tmatesoft.sqljet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT363` | 'org.geotools.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT365` | 'org.cef.network' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT366` | 'antlr.actions.cpp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT367` | 'org.w3.x2000' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT368` | 'net.bytebuddy.description' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT370` | 'org.cometd.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT371` | 'org.h2.value' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT372` | 'org.opengis.referencing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT373` | 'org.antlr.analysis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT374` | 'org.openxml4j.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT375` | 'com.vladium.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT376` | 'net.jini.loader' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT377` | 'org.apache.axiom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT379` | 'org.hamcrest.integration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT382` | 'com.jogamp.gluegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT383` | 'com.reuters.sticapi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT384` | 'com.reuters.ansi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT385` | 'org.opengis.layer' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT386` | 'org.jdom.input' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT387` | 'com.jidesoft.wizard' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT388` | 'org.easymock.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT389` | 'javax.microedition.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT391` | 'net.jini.jeri' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT392` | 'com.graphbuilder.struc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT393` | 'net.jini.id' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT395` | 'com.sun.enterprise' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT396` | 'javassist.tools.web' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT397` | 'org.cometd.bayeux' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT399` | 'jogamp.newt.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT400` | 'javax.xml.xquery' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT402` | 'org.jdom.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT404` | 'org.xml.sax' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT406` | 'junit.extensions.abbot' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT407` | 'org.junit.matchers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT410` | 'org.osgi.service' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT412` | 'org.mockito.configuration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT413` | 'org.eclipse.ui' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT414` | 'org.opengis.go' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT415` | 'org.opengis.sld' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT417` | 'javax.wsdl.factory' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT418` | 'jogamp.opengl.es3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT419` | 'org.apache.wml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT422` | 'org.geotools.catalog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT423` | 'org.mockito.runners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT424` | 'com.ibm.oti' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT425` | 'antlr.collections.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT427` | 'org.slf4j.helpers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT429` | 'org.osgi.framework' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT431` | 'org.apache.xerces' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT432` | 'com.sun.appserv' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT433` | 'org.mockito.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT434` | 'org.tartarus.snowball' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT435` | 'org.cometd.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT436` | 'com.trilead.ssh2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT437` | 'org.hamcrest.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT438` | 'de.regnis.q' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT439` | 'org.h2.fulltext' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT440` | 'org.h2.upgrade' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT441` | 'org.easymock.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT443` | 'ca.odell.glazedlists' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT444` | 'javax.xml.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT446` | 'org.apache.lucene' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT448` | 'javassist.compiler.ast' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT450` | 'com.jidesoft.hints' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT452` | 'org.h2.schema' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT453` | 'org.jdom2.adapters' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT454` | 'org.mortbay.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT456` | 'jogamp.graph.curve' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT457` | 'com.jidesoft.chart' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT458` | 'com.jidesoft.grid' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT460` | 'org.jaxen.saxpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT461` | 'org.slf4j.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT462` | 'jogamp.opengl.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT463` | 'com.jidesoft.gauge' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT464` | 'com.jgoodies.forms' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT465` | 'com.jidesoft.shortcut' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT468` | 'com.icl.saxon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT469` | 'javassist.tools.rmi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT471` | 'org.geotools.referencing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT472` | 'org.opengis.temporal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT473` | 'javassist.util.proxy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT475` | 'org.mortbay.log' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT476` | 'com.google.gson' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT478` | 'org.iso_relax.catalog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT479` | 'com.jidesoft.combobox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT480` | 'org.opengis.parameter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT481` | 'org.geotools.metadata' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT482` | 'jogamp.common.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT483` | 'com.googlecode.javaewah' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT484` | 'org.mockito.creation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT488` | 'net.jini.config' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT489` | 'net.bytebuddy.implementation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT490` | 'org.intellij.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT492` | 'org.objenesis.strategy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT493` | 'org.eclipse.emf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT494` | 'org.cef.callback' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT496` | 'abbot.editor.widgets' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT497` | 'net.bytebuddy.jar' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT498` | 'org.eclipse.elk' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT499` | 'org.opengis.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT500` | 'org.jdom.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT501` | 'net.jini.export' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT504` | 'org.geotools.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT506` | 'org.h2.expression' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT507` | 'org.mortbay.servlet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT509` | 'org.apache.log4j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT512` | 'freemarker.ext.rhino' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT514` | 'jogamp.newt.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT515` | 'com.google.inject' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT518` | 'com.jidesoft.dialog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT519` | 'net.bytebuddy.build' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT522` | 'org.apache.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT524` | 'org.antlr.runtime' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT525` | 'jogamp.opengl.egl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT526` | 'org.geotools.nature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT527` | 'org.junit.runners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT528` | 'com.microsoft.schemas' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT529` | 'org.mortbay.component' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT530` | 'org.apache.neethi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT531` | 'org.apache.tools' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT532` | 'com.reuters.rfa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT537` | 'org.apache.xmpbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT538` | 'org.jdom.adapters' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT539` | 'net.jini.admin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT541` | 'com.sun.jna' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT542` | 'net.jini.url' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT543` | 'org.mortbay.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT544` | 'org.geotools.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT545` | 'org.jaxen.jdom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT546` | 'com.graphbuilder.org' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT549` | 'org.h2.engine' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT550` | 'org.apache.xmlbeans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT551` | 'com.jidesoft.tree' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT552` | 'jp.gr.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT554` | 'org.h2.table' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT555` | 'org.geotools.styling' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT556` | 'org.jdom2.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT558` | 'com.google.thirdparty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT559` | 'com.jidesoft.marker' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT560` | 'org.junit.experimental' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT561` | 'com.sun.el' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT562` | 'org.jdom2.output' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT565` | 'org.geotools.image' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT566` | 'org.jdom.output' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT567` | 'org.hamcrest.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT568` | 'javassist.bytecode.stackmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT569` | 'javax.wsdl.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT570` | 'jogamp.graph.geom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT572` | 'com.thaiopensource.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT573` | 'org.openxml4j.exceptions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT574` | 'jogamp.nativewindow.jawt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT575` | 'org.h2.compress' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT576` | 'net.jini.constraint' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT577` | 'org.w3c.xsl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT578` | 'net.jini.iiop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT579` | 'com.ibm.wsdl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT580` | 'org.jetbrains.annotations' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT581` | 'org.geotools.math' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT582` | 'com.jidesoft.status' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT583` | 'org.easymock.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT584` | 'com.jidesoft.swing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT585` | 'com.silveregg.wrapper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT586` | 'org.jacoco.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT587` | 'org.apache.pdfbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT588` | 'jogamp.opengl.glu' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT591` | 'com.jidesoft.action' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT593` | 'com.reuters.ts1' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT594` | 'org.jaxen.pattern' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT599` | 'org.jaxen.dom4j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT600` | 'net.jpountz.lz4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT602` | 'org.eclipse.jetty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT606` | 'net.bytebuddy.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT607` | 'org.jacoco.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT608` | 'com.jcraft.jsch' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT609` | 'com.jidesoft.tooltip' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT612` | 'org.powermock.classloading' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT613` | 'org.hamcrest.text' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT615` | 'com.jidesoft.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT617` | 'net.jpountz.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT619` | 'org.hamcrest.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT620` | 'org.hamcrest.object' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT621` | 'org.osgi.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT623` | 'com.jidesoft.introspector' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT624` | 'org.mockito.junit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT625` | 'com.reuters.ipc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT626` | 'com.jidesoft.lucene' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT629` | 'javax.xml.parsers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT630` | 'org.relaxng.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT631` | 'com.fasterxml.jackson' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT632` | 'com.jidesoft.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT633` | 'org.cef.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT634` | 'com.jidesoft.docking' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT636` | 'com.sun.org' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT640` | 'net.bytebuddy.agent' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT641` | 'com.reuters.tibmsg' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT642` | 'org.jdesktop.animation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT645` | 'com.jidesoft.plaf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT646` | 'com.sun.mail' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT648` | 'org.apache.poi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT649` | 'javax.servlet.jsp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT650` | 'net.jini.jrmp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT651` | 'abbot.finder.matchers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT652` | 'com.jidesoft.animation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT654` | 'com.jidesoft.tipoftheday' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT656` | 'com.reuters.sass3j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT657` | 'com.reuters.mainloop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT658` | 'com.reuters.ssl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT659` | 'org.powermock.utils' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT660` | 'com.reuters.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT662` | 'jogamp.nativewindow.x11' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT663` | 'com.vladium.emma' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT664` | 'org.junit.rules' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT665` | 'com.jidesoft.comparator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT666` | 'com.jidesoft.document' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT667` | 'org.h2.api' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT668` | 'org.jdom2.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT669` | 'antlr.actions.java' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT673` | 'com.jidesoft.list' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT674` | 'org.apache.fop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT675` | 'freemarker.template.utility' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT676` | 'jogamp.opengl.es1' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT677` | 'org.junit.validator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT678` | 'net.jini.lease' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT679` | 'com.jidesoft.converter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT681` | 'freemarker.ext.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT682` | 'org.iso_relax.jaxp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT684` | 'com.jogamp.nativewindow' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT687` | 'com.thaiopensource.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT690` | 'org.powermock.modules' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT691` | 'org.geotools.ows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT692` | 'net.bytebuddy.pool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT693` | 'com.vladium.app' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT694` | 'com.jidesoft.hssf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT695` | 'com.sun.cdc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT698` | 'com.sun.activation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT701` | 'com.googlecode.concurrentlinkedhashmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT702` | 'com.hp.hpl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT703` | 'com.ibm.icu' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT704` | 'com.microsoft.sqlserver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT705` | 'commonj.sdo.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT706` | 'com.mysql.cj' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT707` | 'com.mysql.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT708` | 'com.orientechnologies.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT709` | 'com.orientechnologies.nio' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT710` | 'com.orientechnologies.orient' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT711` | 'com.sun.istack' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT712` | 'com.sun.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT714` | 'io.jsonwebtoken.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT715` | 'io.jsonwebtoken.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT721` | 'javax.json.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT722` | 'javax.json.stream' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT723` | 'javax.persistence.criteria' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT724` | 'javax.persistence.metamodel' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT725` | 'javax.persistence.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT726` | 'javax.ws.rs' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT727` | 'javax.xml.bind' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT728` | 'junit.extensions.jfcunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT729` | 'junit.extensions.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT730` | 'mssql.googlecode.cityhash' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT731` | 'mssql.googlecode.concurrentlinkedhashmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT732` | 'net.oauth.client' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT733` | 'net.oauth.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT734` | 'net.oauth.signature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT737` | 'org.apache.geronimo' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT738` | 'org.apache.jena' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT739` | 'org.apache.regexp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT740` | 'org.apache.wink' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT741` | 'org.custommonkey.xmlunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT742` | 'org.eclipse.lyo' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT743` | 'org.eclipse.persistence' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT744` | 'org.jdesktop.jxlayer' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT746` | 'org.neo4j.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT747` | 'org.netbeans.jemmy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT748` | 'org.postgresql.copy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT749` | 'org.postgresql.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT750` | 'org.postgresql.ds' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT751` | 'org.postgresql.fastpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT752` | 'org.postgresql.geometric' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT753` | 'org.postgresql.gss' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT754` | 'org.postgresql.hostchooser' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT755` | 'org.postgresql.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT756` | 'org.postgresql.jdbc2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT757` | 'org.postgresql.jdbc3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT758` | 'org.postgresql.largeobject' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT759` | 'org.postgresql.osgi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT760` | 'org.postgresql.ssl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT761` | 'org.postgresql.sspi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT762` | 'org.postgresql.translation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT763` | 'org.postgresql.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT764` | 'org.postgresql.xa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT765` | 'org.springframework.aop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT766` | 'org.springframework.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT767` | 'org.springframework.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT768` | 'org.springframework.boot' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT769` | 'org.springframework.cache' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT770` | 'org.springframework.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT771` | 'org.springframework.context' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT772` | 'org.springframework.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT773` | 'org.springframework.dao' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT774` | 'org.springframework.ejb' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT775` | 'org.springframework.expression' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT776` | 'org.springframework.format' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT777` | 'org.springframework.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT778` | 'org.springframework.instrument' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT779` | 'org.springframework.jca' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT780` | 'org.springframework.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT782` | 'org.springframework.jmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT783` | 'org.springframework.jndi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT784` | 'org.springframework.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT785` | 'org.springframework.ldap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT786` | 'org.springframework.mail' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT788` | 'org.springframework.mock' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT789` | 'org.springframework.objenesis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT790` | 'org.springframework.orm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT792` | 'org.springframework.remoting' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT793` | 'org.springframework.scheduling' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT794` | 'org.springframework.scripting' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT795` | 'org.springframework.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT796` | 'org.springframework.stereotype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT797` | 'org.springframework.test' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT798` | 'org.springframework.transaction' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT799` | 'org.springframework.ui' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT800` | 'org.springframework.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT801` | 'org.springframework.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT802` | 'org.springframework.web' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT803` | 'org.sqlite.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT804` | 'org.sqlite.date' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT805` | 'org.sqlite.javax' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT806` | 'org.sqlite.jdbc3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT807` | 'org.sqlite.jdbc4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT808` | 'org.sqlite.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT812` | 'org.tukaani.xz' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT819` | 'org.eclipse.cdt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT820` | 'org.eclipse.jface' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT821` | 'org.eclipse.swt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT822` | 'org.eclipse.text' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT823` | 'com.zaxxer.sparsebits' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT825` | 'org.abego.treelayout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT826` | 'org.antlr.v4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT827` | 'org.glassfish.json' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT828` | 'org.stringtemplate.v4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT830` | 'org.apache.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT831` | 'org.aspectj.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT832` | 'org.aspectj.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT833` | 'org.h2.mode' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT834` | 'org.slf4j.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT835` | 'org.apache.felix' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT836` | 'org.eclipse.equinox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT837` | 'org.osgi.dto' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT838` | 'org.osgi.resource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT839` | 'jakarta.xml.ws' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT840` | 'oracle.core.lmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT841` | 'oracle.core.lvf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT842` | 'oracle.jdbc.aq' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT843` | 'oracle.jdbc.babelfish' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT844` | 'oracle.jdbc.clio' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT845` | 'oracle.jdbc.connector' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT846` | 'oracle.jdbc.datasource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT847` | 'oracle.jdbc.dcn' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT848` | 'oracle.jdbc.diagnostics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT849` | 'oracle.jdbc.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT850` | 'oracle.jdbc.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT851` | 'oracle.jdbc.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT852` | 'oracle.jdbc.oci' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT853` | 'oracle.jdbc.oracore' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT854` | 'oracle.jdbc.pool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT855` | 'oracle.jdbc.proxy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT856` | 'oracle.jdbc.replay' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT857` | 'oracle.jdbc.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT858` | 'oracle.jdbc.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT859` | 'oracle.jdbc.xa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT860` | 'oracle.jpub.runtime' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT861` | 'oracle.net.ano' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT862` | 'oracle.net.aso' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT863` | 'oracle.net.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT864` | 'oracle.net.jndi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT865` | 'oracle.net.mesg' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT866` | 'oracle.net.ns' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT867` | 'oracle.net.nt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT868` | 'oracle.net.resolver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT869` | 'oracle.security.o3logon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT870` | 'oracle.security.o5logon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT871` | 'oracle.sql.converter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT872` | 'oracle.sql.json' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT873` | 'io.prometheus.client' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT874` | 'com.google.errorprone' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT875` | 'org.postgresql.jdbcurlresolver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT876` | 'org.postgresql.jre7' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT877` | 'org.postgresql.plugin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT878` | 'org.postgresql.replication' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT879` | 'org.postgresql.shaded' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT880` | 'org.postgresql.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT881` | 'io.grpc.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT882` | 'io.grpc.netty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT883` | 'io.grpc.protobuf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT884` | 'io.grpc.stub' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| `JAPIEXT885` | 'io.grpc.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |

### Suggested Improvements (243 checks)

| Check ID | Message |
| -------- | ------- |
| `IMPORTDYN` | Using function syntax to call 'import' is not recommended. With appropriate code changes, use command syntax instead. |
| `LERR` | LASTERR and LASTERROR are not recommended. Use an identifier on the CATCH block instead. |
| `EVLC` | Using 'evalc' with two arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| `RAND` | RAND or RANDN with the 'seed', 'state', or 'twister' inputs is not recommended. Use RNG instead. |
| `HOUGH` | HOUGH(BW,'ThetaResolution',VAL) is not recommended. Use HOUGH(BW,'Theta',-90:VAL:(90-VAL)) instead. |
| `THOUR` | 'hour' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TMNTH` | 'month' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TMNUT` | 'minute' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TNDAY` | 'day' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TSCND` | 'second' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TQURT` | 'quarter' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TYEAR` | 'year' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| `TDTVEC` | 'datevec' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' instead. |
| `TNOW1` | 'now' is not recommended. With appropriate code changes, use 'datetime(\ |
| `TNOW2` | 'datetime(now, 'ConvertFrom', 'datenum')' is not recommended. Use 'datetime(\ |
| `TTDAY1` | 'today' is not recommended. With appropriate code changes, use 'datetime(\ |
| `TTDAY2` | 'datetime(today, 'ConvertFrom', 'datenum')' is not recommended. Use 'datetime(\ |
| `MATCH2` | STRMATCH is not recommended. Use STRNCMP or VALIDATESTRING instead. |
| `IMGDT` | Using 'DataAugmentation' in function 'imageInputLayer' is not recommended. Use function 'augmentedImageDatastore' instead. |
| `MATCH3` | STRMATCH is not recommended. Use STRCMP instead. |
| `MTFA1` | MAKETFORM('AFFINE',A) is not recommended. Use AFFINE2D or AFFINE3D instead. |
| `MTFA2` | MAKETFORM('AFFINE',U,X) is not recommended. Use FITGEOTRANS instead. |
| `MTFP1` | MAKETFORM('PROJECTIVE',A) is not recommended. Use PROJECTIVE2D instead. |
| `MTFP2` | MAKETFORM('PROJECTIVE',U,X) is not recommended. Use FITGEOTRANS instead. |
| `MTFB` | MAKETFORM('BOX',...) is not recommended. Use IMREF2D or IMREF3D instead. |
| `OOPS` | Defining a class using 'function' syntax is not recommended. With appropriate code changes, use 'classdef' syntax instead. |
| `PMTMCONF` | When using PMTM with three output arguments, the 'ConfidenceLevel' input argument is recommended. |
| `NCHKI` | NARGCHK is not recommended. Use NARGINCHK instead. |
| `NCHKO` | Using NARGCHK with NARGOUT is not recommended. Use NARGOUTCHK instead. |
| `NCHKN` | NARGCHK is not recommended. Use NARGINCHK without ERROR instead. |
| `NCHKM` | NARGCHK is not recommended. Use NARGOUTCHK without ERROR instead. |
| `ISCLSTR` | To support string in addition to cellstr, include a call to 'isstring'. |
| `EMTAG` | The compilation directive (or pragma) 'eml' is not recommended. Use 'codegen' instead. |
| `EMXTR` | The 'eml' namespace is not recommended. Use 'codegen' instead. |
| `NVREPLA` | 'addParamValue' is not recommended. Use 'addParameter' instead. |
| `NVREPLM` | 'MidPctRef' is not recommended. Use 'MidPercentReferenceLevel' instead. |
| `NVREPLP` | 'PctRefLevels' is not recommended. Use 'PercentReferenceLevels' instead. |
| `VIDREAD` | 'NumberOfFrames' is not recommended. Use 'NumFrames' instead. |
| `CRNR` | CORNER is not recommended. Use detectHarrisFeatures or detectMinEigenFeatures in Computer Vision Toolbox instead. |
| `CRNRM` | CORNERMETRIC is not recommended. Use detectHarrisFeatures or detectMinEigenFeatures and the cornerPoints class in Computer Vision Toolbox instead. |
| `MDFLT1` | BLKSZ is required for backward compatibility and is ignored. Use [] instead. |
| `INTRPP` | 'pp' is not recommended. Use the griddedInterpolant class instead. |
| `DISPLAYPROG` | Programmatic use of DISPLAY is not recommended. Use DISP or FPRINTF instead. |
| `HGSTGT` | hgsetget is not recommended. Use matlab.mixin.SetGet or matlab.mixin.SetGetExactNames instead. |
| `LEGACYMD` | Setting LegacyMode to true is not recommended. Set LegacyMode to false instead. |
| `LEGACYTRD` | 'DetectorMethod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| `LEGACYTRL` | 'LoopMethod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| `LEGACYTRU` | 'UpdatePeriod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| `LEGACYTRS` | 'StepSize' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| `LEGACYTRG` | 'GainOutputPort' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| `EZPLT` | EZPLOT is not recommended. Use FPLOT or FIMPLICIT instead. |
| `EZGRPH3` | EZGRAPH3 is not recommended. Use FCONTOUR, FMESH, FPLOT, FPLOT3 or FSURF instead. |
| `EZCNTRF` | EZCONTOURF is not recommended. Use FCONTOUR instead, and set the 'Fill' value to 'on'. |
| `EZMSHC` | EZMESHC is not recommended. Use FMESH instead, and set the 'ShowContours' value to 'on'. |
| `EZSRFC` | EZSURFC is not recommended. Use FSURF instead, and set the 'ShowContours' value to 'on'. |
| `FISADR` | 'addrule' is not recommended. Use 'addRule' instead. |
| `STRQUOT` | string('...') is not recommended. Use \ |
| `STRCLQT` | 'string({''str1'', ''str2''})' is not recommended. Use '[\ |
| `SIM` | 'sim' in parfor loop is not recommended. Replace the parfor loop with 'parsim'. |
| `NUMCH` | 'NumberOfChannels' is not recommended. Use 'NumChannels' instead. |
| `GTRED` | 'geotiffread' is not recommended, except when reading a GeoTIFF file from a URL. With appropriate code changes, use 'readgeoraster' instead. |
| `GETFSP` | Using get for retrieving values of line spacing is not recommended. With appropriate code changes, use 'settings' object instead. |
| `SETFMT` | Using set for assigning values of numeric display format is not recommended. With appropriate code changes, use 'settings' object instead. |
| `SETFSP` | Using set for assigning values of line spacing is not recommended. With appropriate code changes, use 'settings' object instead. |
| `GETFMT` | Using get for retrieving values of numeric display format is not recommended. With appropriate code changes, use 'settings' object instead. |
| `EV2IN` | Using 'eval' with two arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| `EV3IN` | Using 'evalin' with three arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| `PRTOG` | '-opengl' is not recommended. Use '-image' instead, which is a direct replacement. |
| `PRTPT` | '-painters' is not recommended. Use '-vector' instead, which is a direct replacement. |
| `FORMATNOI` | 'format' with no input or output arguments is not recommended. Use 'format(\ |
| `XFRWSB` | The 'TransferBaseWorkspaceVariables' option is not recommended for 'batchsim'. With appropriate code changes, consider using project startup scripts or the 'SetupFcn' option instead. |
| `XFRWSP` | The 'TransferBaseWorkspaceVariables' option is not recommended for 'parsim'. With appropriate code changes, consider using project startup scripts or the 'SetupFcn' option instead. |
| `OLDSIM` | This syntax of the 'sim' command which returns multiple arguments is not recommended. With appropriate code changes, turn on 'ReturnWorkspaceOutputs' and return simulation results using the single-output format instead. |
| `INSTHWI` | 'instrhwinfo('ivi')' is not recommended. With appropriate code changes, use 'ividriverlist' or 'ividevlist' instead. |
| `INWVX` | 'instrhwinfo('vxipnp')' is not recommended. With appropriate code changes, use 'ividriverlist' or 'ividevlist' instead. |
| `VERMATLAB` | ver('matlab') is not recommended. With appropriate code changes, use 'matlabRelease' instead. |
| `VERLESSMATLAB` | verLessThan('matlab', ...) is not recommended. With appropriate code changes, use 'isMATLABReleaseOlderThan' instead. |
| `RISKLMM` | 'Model' property of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'UnderlyingModel' property of 'risk.credit.pd.LifetimePDModel' class instead. |
| `RISKLMA` | 'modelAccuracy' method of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.pd.LifetimePDModel' class instead. |
| `RISKLMAP` | 'modelAccuracyPlot' method of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.pd.LifetimePDModel' class instead. |
| `RISKLGDMA` | 'modelAccuracy' method of 'risk.credit.lgd.LGDModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.lgd.LGDModel' class instead. |
| `RISKLGDMAP` | 'modelAccuracyPlot' method of 'risk.credit.lgd.LGDModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.lgd.LGDModel' class instead. |
| `RISKEADMA` | 'modelAccuracy' method of 'risk.credit.ead.EADModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.ead.EADModel' class instead. |
| `RISKEADMAP` | 'modelAccuracyPlot' method of 'risk.credit.ead.EADModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.ead.EADModel' class instead. |
| `HOLDALL` | 'hold('all')' is not recommended. Use 'hold('on')' instead, which is a direct replacement. |
| `MASSO` | 'MasterSolverOptions' is not recommended. Use 'MainSolverOptions' instead, which is a direct replacement. |
| `IMSSO` | 'IntMasterSolverOptions' is not recommended. Use 'IntMainSolverOptions' instead, which is a direct replacement. |
| `CDFEPOCH2DATE` | 'ConvertEpochToDatenum' is not recommended. With appropriate code changes, use the 'DatetimeType' parameter of 'cdfread' instead. |
| `FEATGPID` | 'feature('getpid')' is unsupported and not recommended. With appropriate code changes, use the function 'matlabProcessID' instead. |
| `DTRIREP` | 'TriRep' is not recommended. With appropriate code changes, use 'triangulation' instead. |
| `DDELTRI` | 'DelaunayTri' is not recommended. With appropriate code changes, use 'delaunayTriangulation' instead. |
| `DTRIINT` | 'TriScatteredInterp' is not recommended. With appropriate code changes, use 'scatteredInterpolant' instead. |
| `DAPPLUT` | 'applylut' is not recommended. With appropriate code changes, use 'bwlookup' instead. |
| `DBLKPRC` | 'blkproc' is not recommended. With appropriate code changes, use 'blockproc' instead. |
| `CAXIS` | 'caxis' is not recommended. Use 'clim' instead, which is a direct replacement. |
| `CDFEPOCH` | 'cdfepoch' is not recommended. With appropriate code changes, use 'cdflib' low-level functions instead. |
| `TODATENUM` | 'todatenum' is not recommended. With appropriate code changes, use the 'DatetimeType' parameter of 'cdfread' instead. |
| `COMMPAMM` | 'comm.PAMModulator' is not recommended. With appropriate code changes, use 'pammod' instead. |
| `COMMPAMD` | 'comm.PAMDemodulator' is not recommended. With appropriate code changes, use 'pamdemod' instead. |
| `COMMSRC` | 'commsrc.pn' is not recommended. With appropriate code changes, use 'comm.PNSequence' instead. |
| `DCPTF` | 'cp2tform' is not recommended. With appropriate code changes, use 'fitgeotrans' instead. |
| `DATNM` | 'datenum' is not recommended. With appropriate code changes, use 'datetime' instead. |
| `DATST` | 'datestr' is not recommended. With appropriate code changes, use 'datetime' instead. |
| `DETIM` | 'etime' is not recommended. With appropriate code changes, use 'datetime' and the minus operator instead. |
| `DATOD` | 'addtodate' is not recommended. With appropriate code changes, use 'datetime', 'duration', and the plus operator instead. |
| `CLOCK` | 'clock' is not recommended. With appropriate code changes, use 'datetime(\ |
| `DATE` | 'date' is not recommended. With appropriate code changes, use 'datetime(\ |
| `DATIC` | 'datetick' is not recommended. With appropriate code changes, use datetime and duration arrays directly in charts. Modify display using 'xtickformat', 'ytickformat', or 'ztickformat'. |
| `WKNUM` | 'weeknum' is not recommended. With appropriate code changes, use 'week' with a 'datetime' input instead. |
| `EMDATE` | 'eomdate' is not recommended. With appropriate code changes, use 'dateshift' with a 'datetime' input instead. |
| `XMDATE` | 'x2mdate' is not recommended. With appropriate code changes, use 'datetime(..., \ |
| `MXDATE` | 'm2xdate' is not recommended. With appropriate code changes, use 'exceltime' with a 'datetime' input instead. |
| `MNTHS` | 'months' is not recommended. With appropriate code changes, use 'between' with a 'datetime' input instead. |
| `DGTORD` | 'degtorad' is not recommended. Use 'deg2rad' instead, which is a direct replacement. |
| `RDTODG` | 'radtodeg' is not recommended. Use 'rad2deg' instead, which is a direct replacement. |
| `EZCONTR` | 'ezcontour' is not recommended. With appropriate code changes, use 'fcontour' instead. |
| `EZMESH` | 'ezmesh' is not recommended. With appropriate code changes, use 'fmesh' instead. |
| `EZPLT3` | 'ezplot3' is not recommended. With appropriate code changes, use 'fplot3' instead. |
| `EZSURF` | 'ezsurf' is not recommended. With appropriate code changes, use 'fsurf' instead. |
| `EZPOLAR` | 'ezpolar' is not recommended. With appropriate code changes, use 'fpolarplot' instead. |
| `COMPASS` | 'compass' is not recommended. With appropriate code changes, use 'compassplot' instead. |
| `DFLIPDIM` | 'flipdim' is not recommended. With appropriate code changes, use 'flip' instead. |
| `DSTRMT` | 'str2mat' is not recommended. With appropriate code changes, use 'char' instead. |
| `DSTSTR` | 'setstr' is not recommended. With appropriate code changes, use 'char' instead. |
| `DSTRVCT` | 'strvcat' is not recommended. With appropriate code changes, use 'char' instead. |
| `DISSTR` | 'isstr' is not recommended. With appropriate code changes, use 'ischar' instead. |
| `DFTSMTX` | 'fts2mtx' is not recommended. With appropriate code changes, use 'fts2mat' instead. |
| `FISWRT` | 'writefis' is not recommended. Use 'writeFIS' instead, which is a direct replacement. |
| `FISADM` | 'addmf' is not recommended. With appropriate code changes, use 'addMF' instead. |
| `HIST` | 'hist' is not recommended. With appropriate code changes, use 'histogram' instead. |
| `HISTC` | 'histc' is not recommended. With appropriate code changes, use 'histcounts' instead. |
| `ROSE` | 'rose' is not recommended. With appropriate code changes, use 'polarhistogram' instead. |
| `H5CLS` | 'H5.close' is not recommended and no longer has any effect. There is no simple replacement for this. |
| `H5OPN` | 'H5.open' is not recommended and no longer has any effect. There is no simple replacement for this. |
| `HDFI` | 'hdf5info' is not recommended. With appropriate code changes, use 'h5info' instead. |
| `HDFW` | 'hdf5write' is not recommended. With appropriate code changes, use 'h5write' instead. |
| `HDFR` | 'hdf5read' is not recommended. With appropriate code changes, use 'h5read' instead. |
| `IM2BW` | 'im2bw' is not recommended. With appropriate code changes, use 'imbinarize' instead. |
| `ISPIX` | 'pixelLabelImageSource' is not recommended. Use 'pixelLabelImageDatastore' instead, which is a direct replacement. |
| `ISAUG` | 'augmentedImageSource' is not recommended. Use 'augmentedImageDatastore' instead, which is a direct replacement. |
| `ISDNS` | 'denoisingImageSource' is not recommended. Use 'denoisingImageDatastore' instead, which is a direct replacement. |
| `IMFREEH` | 'imfreehand' is not recommended. With appropriate code changes, use 'drawfreehand' instead. |
| `IMRECT` | 'imrect' is not recommended. With appropriate code changes, use 'drawrectangle' instead. |
| `IMLINE` | 'imline' is not recommended. With appropriate code changes, use 'drawline' instead. |
| `IMPNT` | 'impoint' is not recommended. With appropriate code changes, use 'drawpoint' instead. |
| `IMPOLY` | 'impoly' is not recommended. With appropriate code changes, use 'drawpolygon' or 'drawpolyline' instead. |
| `IMELLPS` | 'imellipse' is not recommended. With appropriate code changes, use 'drawellipse' or 'drawcircle' instead. |
| `DIMTRNS` | 'imtransform' is not recommended. With appropriate code changes, use 'imwarp' instead. |
| `FILEATTRIB` | 'fileattrib' is not recommended. With appropriate code changes, use 'filePermissions' instead. |
| `CSVRD` | 'csvread' is not recommended. With appropriate code changes, use 'readtable' or 'readmatrix' instead. |
| `DLMRD` | 'dlmread' is not recommended. With appropriate code changes, use 'readtable' or 'readmatrix' instead. |
| `CSVWT` | 'csvwrite' is not recommended. With appropriate code changes, use 'writematrix' instead. |
| `DLMWT` | 'dlmwrite' is not recommended. With appropriate code changes, use 'writematrix' instead. |
| `XLSRD` | 'xlsread' is not recommended. With appropriate code changes, use 'readtable', 'readmatrix' or 'readcell' instead. |
| `XLSWT` | 'xlswrite' is not recommended. With appropriate code changes, use 'writematrix' or 'writecell' instead. |
| `ISDIR` | 'isdir' is not recommended. Use 'isfolder' instead, which is a direct replacement. |
| `DISEQN` | 'isequalwithequalnans' is not recommended. With appropriate code changes, use 'isequaln' instead. |
| `DGCAT` | 'gcat' is not recommended. Use 'spmdCat' instead, which is a direct replacement. |
| `DGOP` | 'gop' is not recommended. Use 'spmdReduce' instead, which is a direct replacement. |
| `DGPLUS` | 'gplus' is not recommended. Use 'spmdPlus' instead, which is a direct replacement. |
| `DLABBARRIER` | 'labBarrier' is not recommended. Use 'spmdBarrier' instead, which is a direct replacement. |
| `DLABBROADCAST` | 'labBroadcast' is not recommended. Use 'spmdBroadcast' instead, which is a direct replacement. |
| `DLABINDEX` | 'labindex' is not recommended. Use 'spmdIndex' instead, which is a direct replacement. |
| `DLABPROBE` | 'labProbe' is not recommended. Use 'spmdProbe' instead, which is a direct replacement. |
| `DLABRECEIVE` | 'labReceive' is not recommended. Use 'spmdReceive' instead, which is a direct replacement. |
| `DLABSEND` | 'labSend' is not recommended. Use 'spmdSend' instead, which is a direct replacement. |
| `DLABSENDRECEIVE` | 'labSendReceive' is not recommended. Use 'spmdSendReceive' instead, which is a direct replacement. |
| `DNUMLABS` | 'numlabs' is not recommended. Use 'spmdSize' instead, which is a direct replacement. |
| `AGRED` | 'arcgridread' is not recommended. With appropriate code changes, use 'readgeoraster' instead. |
| `MDFOBJ` | 'mdf' is not recommended. With appropriate code changes, use 'mdfInfo', 'mdfChannelGroupInfo' or 'mdfChannelInfo' instead. |
| `DDBMEX` | 'mexdebug' is not recommended. With appropriate code changes, use 'dbmex' instead. |
| `MLNT` | 'mlint' is not recommended. Use 'checkcode' instead, which is a direct replacement. |
| `MNRFIT` | 'mnrfit' is not recommended. With appropriate code changes, use 'fitmnr' instead. |
| `MNRVAL` | 'mnrval' is not recommended. With appropriate code changes, use 'MultinomialRegression.predict' instead. |
| `MUSTINRANGE` | 'mustBeInRange' is not recommended. With appropriate code changes, use 'mustBeBetween' instead. |
| `DEPNOE` | 'numberofelements' is not recommended. With appropriate code changes, use 'numel' instead. |
| `GAOPT` | 'gaoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| `PSOPT` | 'psoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| `SAOPT` | 'saoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| `PLOTYY` | 'plotyy' is not recommended. With appropriate code changes, use 'yyaxis' instead. |
| `POLAR` | 'polar' (MATLAB) is not recommended. Use 'polarplot' instead. |
| `PLBL` | 'polybool' is not recommended. With appropriate code changes, use 'polyshape' instead. |
| `PYVER` | 'pyversion' is not recommended. With appropriate code changes, use 'pyenv' instead. |
| `DQUAD` | 'quad' is not recommended. With appropriate code changes, use 'integral' instead. |
| `DQUADL` | 'quadl' is not recommended. With appropriate code changes, use 'integral' instead. |
| `DQUADV` | 'quadv' is not recommended. With appropriate code changes, use 'integral' instead. |
| `DDBLQD` | 'dblquad' is not recommended. With appropriate code changes, use 'integral2' instead. |
| `DTRIQD` | 'triplequad' is not recommended. With appropriate code changes, use 'integral3' instead. |
| `DFIRRCOS` | 'firrcos' is not recommended. With appropriate code changes, use 'rcosdesign' instead. |
| `DFIRGAUSS` | 'firgauss' is not recommended. With appropriate code changes, use 'gaussdesign' instead. |
| `DGAUSSFIR` | 'gaussfir' is not recommended. With appropriate code changes, use 'gaussdesign' instead. |
| `ROIFILL` | 'roifill' is not recommended. With appropriate code changes, use 'regionfill' instead. |
| `DEPBART` | 'sigwin.barthannwin' is not recommended. With appropriate code changes, use 'barthannwin' instead. |
| `DEPLETT` | 'sigwin.bartlett' is not recommended. With appropriate code changes, use 'bartlett' instead. |
| `DBLKMN` | 'sigwin.blackman' is not recommended. With appropriate code changes, use 'blackman' instead. |
| `DBHRRS` | 'sigwin.blackmanharris' is not recommended. With appropriate code changes, use 'blackmanharris' instead. |
| `DBHMNWN` | 'sigwin.bohmanwin' is not recommended. With appropriate code changes, use 'bohmanwin' instead. |
| `DCHBWN` | 'sigwin.chebwin' is not recommended. With appropriate code changes, use 'chebwin' instead. |
| `DFLTTPWN` | 'sigwin.flattopwin' is not recommended. With appropriate code changes, use 'flattopwin' instead. |
| `DGSWIN` | 'sigwin.gausswin' is not recommended. With appropriate code changes, use 'gausswin' instead. |
| `DHMMNG` | 'sigwin.hamming' is not recommended. With appropriate code changes, use 'hamming' instead. |
| `DHANN` | 'sigwin.hann' is not recommended. With appropriate code changes, use 'hann' instead. |
| `DKSER` | 'sigwin.kaiser' is not recommended. With appropriate code changes, use 'kaiser' instead. |
| `DNLWN` | 'sigwin.nuttallwin' is not recommended. With appropriate code changes, use 'nuttallwin' instead. |
| `DPNWN` | 'sigwin.parzenwin' is not recommended. With appropriate code changes, use 'parzenwin' instead. |
| `DRCTWN` | 'sigwin.rectwin' is not recommended. With appropriate code changes, use 'rectwin' instead. |
| `DTYLRWN` | 'sigwin.taylorwin' is not recommended. With appropriate code changes, use 'taylorwin' instead. |
| `DTRNG` | 'sigwin.triang' is not recommended. With appropriate code changes, use 'triang' instead. |
| `DTKYWN` | 'sigwin.tukeywin' is not recommended. With appropriate code changes, use 'tukeywin' instead. |
| `DBURG` | 'spectrum.burg' is not recommended. With appropriate code changes, use 'pburg' instead. |
| `DCOV` | 'spectrum.cov' is not recommended. With appropriate code changes, use 'pcov' instead. |
| `DEVCTR` | 'spectrum.eigenvector' is not recommended. With appropriate code changes, use 'peig' instead. |
| `DMCOV` | 'spectrum.mcov' is not recommended. With appropriate code changes, use 'pmcov' instead. |
| `DMTM` | 'spectrum.mtm' is not recommended. With appropriate code changes, use 'pmtm' instead. |
| `DMUSIC` | 'spectrum.music' is not recommended. With appropriate code changes, use 'pmusic' instead. |
| `DPRDGRM` | 'spectrum.periodogram' is not recommended. With appropriate code changes, use 'periodogram' instead. |
| `DWELCH` | 'spectrum.welch' is not recommended. With appropriate code changes, use 'pwelch' instead. |
| `DYULEAR` | 'spectrum.yulear' is not recommended. With appropriate code changes, use 'pyulear' instead. |
| `SIMVARIANT` | 'Simulink.Variant' is not recommended. Use 'Simulink.VariantExpression' instead, which is a direct replacement. |
| `COMBNK` | 'combnk' is not recommended. With appropriate code changes, use 'nchoosek' instead. |
| `NANMEAN` | 'nanmean' is not recommended. With appropriate code changes, use 'mean' instead. |
| `NANMEDIAN` | 'nanmedian' is not recommended. With appropriate code changes, use 'median' instead. |
| `NANMAX` | 'nanmax' is not recommended. With appropriate code changes, use 'max' instead. |
| `NANMIN` | 'nanmin' is not recommended. With appropriate code changes, use 'min' instead. |
| `NANSTD` | 'nanstd' is not recommended. With appropriate code changes, use 'std' instead. |
| `NANVAR` | 'nanvar' is not recommended. With appropriate code changes, use 'var' instead. |
| `NANCOV` | 'nancov' is not recommended. With appropriate code changes, use 'cov' instead. |
| `NANSUM` | 'nansum' is not recommended. With appropriate code changes, use 'sum' instead. |
| `CELLDTSET` | 'cell2dataset' is not recommended. With appropriate code changes, use 'cell2table' instead. |
| `DTSET` | 'dataset' is not recommended. With appropriate code changes, use 'table' instead. |
| `MATDTSET` | 'mat2dataset' is not recommended. With appropriate code changes, use 'array2table' instead. |
| `STRUCTDTSET` | 'struct2dataset' is not recommended. With appropriate code changes, use 'struct2table' instead. |
| `FSTR` | 'findstr' is not recommended. With appropriate code changes, use 'strfind' instead. |
| `DSTRRD` | 'strread' is not recommended. With appropriate code changes, use 'textscan' instead. |
| `DTXTRD` | 'textread' is not recommended. With appropriate code changes, use 'textscan' instead. |
| `SUBIMGNR` | 'subimage' is not recommended. With appropriate code changes, use 'imshow' instead. |
| `URLWR` | 'urlwrite' is not recommended. With appropriate code changes, use 'websave' instead. |
| `URLRD` | 'urlread' is not recommended. With appropriate code changes, use 'webread' or 'webwrite' instead. |
| `VEMAT` | 'vec2mat' is not recommended. With appropriate code changes, use 'reshape' instead. |
| `MDFRD` | 'read' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfRead' function instead. |
| `MDFCHL` | 'channelList' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfChannelInfo' function instead. |
| `MDFSVA` | 'saveAttachment' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfSaveAttachment' function instead. |
| `MINELV` | 'MinElevationAngle' is not recommended. Use 'MaskElevationAngle' instead, which is a direct replacement when specified as scalar values. Row vector values must be transposed. |

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
