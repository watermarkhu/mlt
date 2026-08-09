---
icon: lucide/list-checks
---

# Rules Reference

## Overview

mlt implements lint rules matching MATLAB's Code Analyzer checks for feature parity. Rules use the same check IDs as MATLAB (e.g., `NOSEMI`, `AGROW`, `naming.class.casing`) and are organized into categories.

## Rule Table

| Rule ID | Category | Description | Default Severity | Auto-fix |
| ------- | -------- | ----------- | ---------------- | -------- |
| [NOSEMI](nosemi.md) | Formatting | Statement without trailing semicolon may produce unintended console output | Info | Yes |
| [MFAMB](mfamb.md) | Readability | Cannot determine whether name is a variable or function; assumes function | Info | No |
| [FLUDLR](fludlr.md) | Readability | Nested `flipud(fliplr(x))`/`fliplr(flipud(x))` should use `rot90(x, 2)` | Info | Yes |
| [STLOW](stlow.md) | Readability | Unnecessary UPPER/LOWER call in a comparison | Info | Yes |
| [COMNL](comnl.md) | Readability | Newline following comma acts as a row separator in a matrix; suggest semicolon or ellipsis | Info | Yes |
| [FVINR](fvinr.md) | Readability | For readability, add Input attribute to the input arguments block | Info | Yes |
| [NOFIL](nofil.md) | Incomplete Analysis | File not found | Error | No |
| [RDERR](rderr.md) | Incomplete Analysis | Unable to read file | Error | No |
| [QUIT](quit.md) | Incomplete Analysis | Code analysis did not complete; the analyzer encountered an internal error | Error | No |
| [NOPAR2](nopar2.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at `(` on a line | Error | No |
| [EOLPAR](eolpar.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of line | Error | No |
| [ENDPAR](endpar.md) | Syntax Errors | A `(` might be missing a closing `)`, causing invalid syntax at end of file | Error | No |
| [UNSET](unset.md) | Syntax Errors | Invalid use of operator on the left side of an assignment | Error | No |
| [LHROW](lhrow.md) | Syntax Errors | The left side of an assignment cannot have multiple rows (';') | Error | No |
| [RESWD](reswd.md) | Syntax Errors | Invalid use of a reserved word | Error | No |
| [SYNEND](synend.md) | Syntax Errors | Invalid use for END operator | Error | No |
| [MCPLD](mcpld.md) | Syntax Errors | Invalid property syntax | Error | No |
| [BADNOT](badnot.md) | Syntax Errors | Using ~ to ignore a value is not permitted in this context | Error | No |
| [BADNOTLHS](badnotlhs.md) | Syntax Errors | Invalid use of logical not operator (~) on left side of an assignment | Error | No |
| [BADCT](badct.md) | Syntax Errors | Unicode explicit directional formatting characters are not supported | Error | No |
| [ENDCT2](endct2.md) | Syntax Errors | An END might be missing after a block-opening keyword | Error | No |
| [ENDCT3](endct3.md) | Syntax Errors | An END might be missing before a block-opening keyword | Error | No |
| [ENDCT4](endct4.md) | Syntax Errors | A METHODS block or END might be missing before a function definition | Error | No |
| [STRIN](strin.md) | Syntax Errors | A quoted character vector is unterminated | Error | No |
| [DOUQT](douqt.md) | Syntax Errors | A double quoted string is unterminated | Error | No |
| [INBLK](inblk.md) | Syntax Errors | A block comment is unterminated at the end of the file | Error | No |
| [BADFP](badfp.md) | Syntax Errors | Invalid floating-point constant | Error | No |
| [BADHBH](badhbh.md) | Syntax Errors | Invalid digit in hexadecimal literal | Error | No |
| [BADHBB](badhbb.md) | Syntax Errors | Invalid digit in binary literal | Error | No |
| [BADHBHT](badhbht.md) | Syntax Errors | Hexadecimal literal has too many digits for specified type suffix | Error | No |
| [BADHBBT](badhbbt.md) | Syntax Errors | Binary literal has too many digits for specified type suffix | Error | No |
| [HEXTOOLONG](hextoolong.md) | Syntax Errors | Hexadecimal literal has too many digits | Error | No |
| [BINARYTOOLONG](binarytoolong.md) | Syntax Errors | Binary literal has too many digits | Error | No |
| [SBTMP](sbtmp.md) | Syntax Errors | Invalid array indexing or function call; chaining outputs after parenthesis is not supported | Error | No |
| [FVSYN](fvsyn.md) | Syntax Errors | Invalid function argument syntax | Error | No |
| [FVACI](fvaci.md) | Syntax Errors | Use of name-value arguments in cell indexing is not supported | Error | No |
| [FVACS](fvacs.md) | Syntax Errors | Using a character vector or string as a name in name=value syntax is not supported | Error | No |
| [FVAMI](fvami.md) | Syntax Errors | Name in name-value argument syntax must be a valid MATLAB identifier | Error | No |
| [VTPOD](vtpod.md) | Syntax Errors | Specify validation in the following order: size, then class, then functions | Error | No |
| [FVAPN](language-spec.md) | Language Specification | Move name-value arguments that use the name=value syntax to the end of the argument list | Error | No |
| [FVATF](language-spec.md) | Language Specification | Attribute values in arguments blocks must be logical constants | Error | No |
| [FVBTN](language-spec.md) | Language Specification | Use of this function is not supported in arguments blocks | Error | No |
| [FVDAN](language-spec.md) | Language Specification | Same name as both a name-value argument structure and a positional argument | Error | No |
| [FVDAP](language-spec.md) | Language Specification | Positional argument can only be declared once | Error | No |
| [FVDNF](language-spec.md) | Language Specification | Name-value argument can only be declared once | Error | No |
| [FVDREP](language-spec.md) | Language Specification | Multiple Repeating arguments blocks are not supported | Error | No |
| [FVIDV](language-spec.md) | Language Specification | Specifying validation or default value for ignored arguments is not supported | Error | No |
| [FVIOA](language-spec.md) | Language Specification | Both 'Input' and 'Output' attributes on the same arguments block is not supported | Error | No |
| [FVMCL](language-spec.md) | Language Specification | Multiple name-value structures using .? syntax and a class name is not supported | Error | No |
| [FVNDE](language-spec.md) | Language Specification | Default values for class-name name-value arguments are illegal | Error | No |
| [FVNIV](language-spec.md) | Language Specification | Variable is not an input to the function and cannot be used in an arguments block | Error | No |
| [FVNREP](language-spec.md) | Language Specification | Name-value arguments are not supported in a Repeating arguments block | Error | No |
| [FVNSC](language-spec.md) | Language Specification | Use of nested functions is not supported in arguments blocks | Error | No |
| [FVNVL](language-spec.md) | Language Specification | Validation for class-name name-value arguments is illegal | Error | No |
| [FVOBI](language-spec.md) | Language Specification | Declare all input argument blocks before all output arguments blocks | Error | No |
| [FVOCON](language-spec.md) | Language Specification | For output arguments, validation functions must only use the argument being validated or literals | Error | No |
| [FVOND](language-spec.md) | Language Specification | Use of name-value arguments in default values is not supported | Error | No |
| [FVONV](language-spec.md) | Language Specification | Use of name-value arguments without dotted name in the validation is not supported | Error | No |
| [FVOOD](language-spec.md) | Language Specification | Specifying a default value for an output argument is not supported | Error | No |
| [FVOOI](language-spec.md) | Language Specification | Use of ignored arguments in output arguments block is not supported | Error | No |
| [FVOON](language-spec.md) | Language Specification | Using name-value argument as output argument is not supported | Error | No |
| [FVORDI](language-spec.md) | Language Specification | Ignored input arguments are not allowed after a Repeating arguments block or name-value arguments | Error | No |
| [FVORDN](language-spec.md) | Language Specification | Positional arguments must be defined before name-value arguments | Error | No |
| [FVORDO](language-spec.md) | Language Specification | Repeating output arguments must be defined after required output arguments | Error | No |
| [FVORDP](language-spec.md) | Language Specification | Positional arguments must be defined in the following order: required, optional, and repeating | Error | No |
| [FVORM](language-spec.md) | Language Specification | Declaring multiple repeating output arguments is not supported | Error | No |
| [FVOVREP](language-spec.md) | Language Specification | Output argument varargout can only be used inside a Repeating output arguments block | Error | No |
| [FVREPD](language-spec.md) | Language Specification | Default values are not supported in a Repeating arguments block | Error | No |
| [FVREPO](language-spec.md) | Language Specification | Repeating input arguments block containing varargin must not have other arguments | Error | No |
| [FVSOR](language-spec.md) | Language Specification | Input arguments block declarations and the function line must contain the same input arguments in the same order | Error | No |
| [FVSORO](language-spec.md) | Language Specification | Output arguments block declarations and the function line must contain the same output arguments in the same order | Error | No |
| [FVUBD](language-spec.md) | Language Specification | Argument is referenced before it is declared in the arguments block | Error | No |
| [FVVCON](language-spec.md) | Language Specification | For input arguments, validation functions must only use previously declared positional arguments, the argument being validated, or literals | Error | No |
| [FVVIN](language-spec.md) | Language Specification | Validation function must use the argument as an input | Error | No |
| [FVVREP](language-spec.md) | Language Specification | varargin can only be used inside repeating input arguments block | Error | No |
| [TINVALDIM](language-spec.md) | Language Specification | Each dimension must be a nonnegative integer number or a colon | Error | No |
| [TTOOFEWDIMS](language-spec.md) | Language Specification | Specify at least two dimensions for size | Error | No |
| [CTOINE](language-spec.md) | Language Specification | Use of constructed object as input to constructor is not supported | Error | No |
| [CTORO](language-spec.md) | Language Specification | Class constructors must be declared with at least one output argument | Error | No |
| [ERTXT](language-spec.md) | Language Specification | Specify an error message with the message identifier | Error | No |
| [MHERIT](language-spec.md) | Language Specification | Deriving from a built-in MATLAB class is not supported | Error | No |
| [NCHKOS](language-spec.md) | Language Specification | NARGINCHK does not return any values | Error | No |
| [SPBFN](language-spec.md) | Language Specification | Use of a workspace-access function inside an SPMD block | Error | No |
| [SPBRK](language-spec.md) | Language Specification | break/continue not fully contained in an SPMD block (subsumed by SPRET) | Error | No |
| [SPDEC](language-spec.md) | Language Specification | SPMD worker bounds must be nonnegative integers | Error | No |
| [SPDEC3](language-spec.md) | Language Specification | SPMD can only specify lower and upper worker bounds | Error | No |
| [SPEVC](language-spec.md) | Language Specification | EVALIN/ASSIGNIN('caller') invalid inside an SPMD block | Error | No |
| [SPLD](language-spec.md) | Language Specification | Assign the output of LOAD to a variable in SPMD blocks | Error | No |
| [SPNF](language-spec.md) | Language Specification | Nested function call inside an SPMD block | Error | No |
| [SPSV](language-spec.md) | Language Specification | SAVE requires the '-fromstruct' option inside an SPMD block | Error | No |
| [SPWHOS](language-spec.md) | Language Specification | who/whos without '-file' is invalid inside an SPMD block | Error | No |
| [USESWNS](language-spec.md) | Language Specification | Variable must be explicitly defined before first use | Error | No |
| [WTXT](language-spec.md) | Language Specification | Specify a warning message with the message identifier | Error | No |
| [MABSEAC](language-spec.md) | Language Specification | Instance properties and methods are illegal in classes that are both Sealed and Abstract | Error | No |
| [MABSEAM](language-spec.md) | Language Specification | A method cannot be both Abstract and Sealed | Error | No |
| [MCAPP](language-spec.md) | Language Specification | Private property cannot be Abstract | Error | No |
| [MCCBS](language-spec.md) | Language Specification | A superclass constructor is being called, but the name is not a declared superclass name | Error | No |
| [MCCBU](language-spec.md) | Language Specification | This superclass constructor is called after a use of the constructed object | Error | No |
| [MCCMC](language-spec.md) | Language Specification | Constructor for superclass can only be called once | Error | No |
| [MCCSOP](language-spec.md) | Language Specification | Unable to modify Constant property | Error | No |
| [MCGSA](language-spec.md) | Language Specification | Method tries to set or get an abstract property | Error | No |
| [MCMIO](language-spec.md) | Language Specification | Method has too many inputs or outputs | Error | No |
| [MCMSP](language-spec.md) | Language Specification | Private method cannot be Abstract | Error | No |
| [MCMTP](language-spec.md) | Language Specification | TestParameterDefinition methods must be Static | Error | No |
| [MCPIN](language-spec.md) | Language Specification | Unable to initialize class property to an instance of the class itself | Error | No |
| [MCPSG](language-spec.md) | Language Specification | Set or get method must be fully defined in the class definition file | Error | No |
| [MCSCC](language-spec.md) | Language Specification | To call the superclass constructor, the subclass constructor name must match the subclass name | Error | No |
| [MCSCF](language-spec.md) | Language Specification | A superclass constructor must be assigned to the first constructor output argument | Error | No |
| [MCSCM](language-spec.md) | Language Specification | To call a superclass method, the method name must match the subclass method name | Error | No |
| [MCSCN](language-spec.md) | Language Specification | Method tries to set a constant property | Error | No |
| [MCSCO](language-spec.md) | Language Specification | A superclass constructor must be called using the first constructor output argument | Error | No |
| [MCSCT](language-spec.md) | Language Specification | Superclass constructor call must not be conditionalized or be part of another expression | Error | No |
| [MCSMO](language-spec.md) | Language Specification | Returning multiple outputs from a superclass object initialization is not supported | Error | No |
| [MCSWA](language-spec.md) | Language Specification | A sealed class cannot specify allowed subclasses | Error | No |
| [MTAGS3](language-spec.md) | Language Specification | Cannot use the Access attribute when using the SetAccess or GetAccess attribute | Error | No |
| [MTMAT](language-spec.md) | Language Specification | Attribute can only be set once | Error | No |
| [MWKCL](language-spec.md) | Language Specification | A WeakHandle property must restrict its type using a class validation | Error | No |
| [MWKCT](language-spec.md) | Language Specification | Specifying both WeakHandle and Constant attributes on the same property is not supported | Error | No |
| [MWKREF](language-spec.md) | Language Specification | Specifying both WeakHandle and Dependent attributes is invalid | Error | No |
| [FPFORP](language-spec.md) | Language Specification | 'fprintf' is writing to a file opened with read permission only | Error | No |
| [FWFORP](language-spec.md) | Language Specification | 'fwrite' is writing to a file opened with read permission only | Error | No |
| [PFANON](language-spec.md) | Language Specification | Sliced output variable used in an anonymous function is not supported in parfor loops | Error | No |
| [PFANSLP](language-spec.md) | Language Specification | 'ans' is not supported as a parfor loop variable | Error | No |
| [PFANSNS](language-spec.md) | Language Specification | 'ans' is not supported as a for loop variable in parfor loops | Error | No |
| [PFCEL](language-spec.md) | Language Specification | The function does not support cell arrays (argument N) | Error | No |
| [PFCTXT](language-spec.md) | Language Specification | Sliced variable indexed with a nested for loop variable must be inside the for loop that defines its range | Error | No |
| [PFEVC](language-spec.md) | Language Specification | EVALIN('caller') and ASSIGNIN('caller') are invalid inside a PARFOR loop | Error | No |
| [PFFRNG](language-spec.md) | Language Specification | When indexing a sliced variable with a nested for loop variable, the range must be a row vector of positive constant numbers | Error | No |
| [PFFSUB](language-spec.md) | Language Specification | Indexing a nested for loop variable is not supported in parfor loops | Error | No |
| [PFINCR](language-spec.md) | Language Specification | Using different reduction functions with the same reduction variable is not supported in parfor loops | Error | No |
| [PFINPT](language-spec.md) | Language Specification | 'inputname' is not supported in parfor loops | Error | No |
| [PFLD](language-spec.md) | Language Specification | 'load' must assign to an output variable in parfor loops | Error | No |
| [PFMLTI](language-spec.md) | Language Specification | The nested for loop variable must not be assigned other than by its for statement | Error | No |
| [PFNACK](language-spec.md) | Language Specification | 'narginchk' and 'nargoutchk' cannot be used in parfor loops | Error | No |
| [PFNAIO](language-spec.md) | Language Specification | 'nargin' and 'nargout' require a function argument in parfor loops | Error | No |
| [PFNAR](language-spec.md) | Language Specification | Subtracting reduction variable from expressions is not supported in parfor loops | Error | No |
| [PFRFH](language-spec.md) | Language Specification | The PARFOR reduction function must be a function name or a broadcast variable | Error | No |
| [PFRNG](language-spec.md) | Language Specification | The range of a PARFOR statement must be increasing consecutive integers | Error | No |
| [PFSLO](language-spec.md) | Language Specification | Variable is indexed using the parfor loop variable, but it is not a valid sliced output variable | Error | No |
| [PFSLRD](language-spec.md) | Language Specification | Invalid combination of sliced indexing and non-indexed reads of a sliced output variable | Error | No |
| [PFSLW](language-spec.md) | Language Specification | Multiple sliced accesses must all use the same list of subscripts | Error | No |
| [PFSV](language-spec.md) | Language Specification | SAVE cannot be called in a PARFOR loop without the '-fromstruct' option | Error | No |
| [PFUNK](language-spec.md) | Language Specification | The PARFOR loop cannot run due to the way variable VAR_NAME is used | Error | No |
| [PFUTMP](language-spec.md) | Language Specification | Temporary variable VAR_NAME must be set inside the parfor loop before it is used | Error | No |
| [PFUTVR](language-spec.md) | Language Specification | Variable VAR_NAME may have been intended as a reduction variable, but is an uninitialized temporary | Error | No |
| [PFVARS](language-spec.md) | Language Specification | Parfor loop contains too many variables | Error | No |
| [PFVSUB](language-spec.md) | Language Specification | Indexing parfor loop variables is not supported in parfor loops | Error | No |

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
4. Add documentation in `docs/agrow.md`

No manual edits to `lib.rs` are needed beyond adding the `pub mod` declaration.

See the [repository](https://github.com/watermarkhu/mlt) for the full development guide.
