---
icon: lucide/book-marked
---

# Language Specification

**Default severity:** Error
**Auto-fix:** No
**Category:** Language Specification
**Can be disabled:** Yes

## What these checks do

These checks validate `arguments` blocks and their interaction with the function signature, matching MATLAB Code Analyzer's function validation checks (FV\* family plus the size checks `TINVALDIM` and `TTOOFEWDIMS`).

The checks operate on three groups:

- **Structure** — block attributes, repeating blocks, output blocks, and class-name name-value arguments.
- **Ordering** — argument declaration order, duplicates, and consistency with the function line.
- **Validation** — validation function rules and size constraints.

## Check IDs

### Structure

| Check ID | Message |
| -------- | ------- |
| `FVDREP` | Multiple Repeating arguments blocks are not supported. |
| `FVIOA` | Specifying both 'Input' and 'Output' attributes on the same arguments block is not supported. |
| `FVREPO` | Repeating input arguments block containing varargin must not have other arguments. |
| `FVVREP` | varargin can only be used inside repeating input arguments block. |
| `FVOVREP` | Output argument varargout can only be used inside a Repeating output arguments block. |
| `FVNREP` | Name-value arguments are not supported in a Repeating arguments block. |
| `FVREPD` | Default values are not supported in a Repeating arguments block. |
| `FVOOD` | Specifying a default value for an output argument is not supported. |
| `FVOOI` | Use of ignored arguments in output arguments block is not supported. |
| `FVOON` | Using name-value argument as output argument is not supported. |
| `FVORM` | Declaring multiple repeating output arguments is not supported. |
| `FVOBI` | Declare all input argument blocks before all output arguments blocks. |
| `FVATF` | Attribute values in arguments blocks must be logical constants. |
| `FVMCL` | Specifying multiple name-value structures using .? syntax and a class name is not supported. |
| `FVNDE` | When specifying name-value arguments using a class name, it is illegal to specify default values for the arguments. |
| `FVNVL` | When specifying name-value arguments using a class name, it is illegal to specify validation for the arguments. |

### Ordering and consistency

| Check ID | Message |
| -------- | ------- |
| `FVORDI` | Ignored input arguments are not allowed after a Repeating arguments block or name-value arguments. |
| `FVORDN` | Positional arguments must be defined before name-value arguments. |
| `FVORDP` | Positional arguments must be defined in the following order: required, optional, and repeating. |
| `FVORDO` | Repeating output arguments must be defined after required output arguments. |
| `FVAPN` | Move name-value arguments that use the name=value syntax to the end of the argument list. |
| `FVDAN` | Using the same name as both a name-value argument structure and as a positional argument is not supported. |
| `FVDAP` | Positional argument can only be declared once. |
| `FVDNF` | Name-value argument can only be declared once. |
| `FVSOR` | Input arguments block declarations and the function line must contain the same input arguments in the same order, including ignored arguments. |
| `FVSORO` | Output arguments block declarations and the function line must contain the same output arguments in the same order. |
| `FVNIV` | This variable is not an input to the function and cannot be used in an arguments block. |
| `FVIDV` | Specifying validation or default value for ignored arguments is not supported. |

### Validation functions and size

| Check ID | Message |
| -------- | ------- |
| `FVVIN` | Validation function must use the argument as an input. |
| `FVVCON` | For input arguments, validation functions must only use previously declared positional arguments, the argument being validated, or literals. |
| `FVOCON` | For output arguments, validation functions must only use the argument being validated or literals. |
| `FVUBD` | Argument is referenced before it is declared in the arguments block. |
| `FVOND` | Use of name-value arguments in default values is not supported. |
| `FVONV` | Use of name-value arguments without dotted name in the validation is not supported. |
| `FVBTN` | Use of this function is not supported in arguments blocks. |
| `FVNSC` | Use of nested functions is not supported in arguments blocks. |
| `TINVALDIM` | Each dimension must be a nonnegative integer number or a colon. |
| `TTOOFEWDIMS` | Specify at least two dimensions for size. |

## Why these checks matter

- **Arguments blocks** are the primary mechanism for input/output validation in modern MATLAB; malformed blocks cause runtime errors at the call site.
- **Declaration order** rules exist because MATLAB resolves positional, optional, repeating, and name-value arguments left-to-right; violations are rejected by the interpreter.
- **Validation function restrictions** (using only the argument, previously declared positionals, and literals) keep validation deterministic and side-effect free.
- **Size constraint checks** catch dimension declarations that can never match a concrete array.

## Examples

### Correct

```matlab
function y = f(a, b, opts)
    arguments
        a (1,1) double
        b = 3
        opts.Name {mustBeReal}
    end
    y = a + b;
end

function [out, varargout] = g(x)
    arguments
        x (1,:) double {mustBeReal}
    end
    arguments (Output)
        out (1,1) double
    end
    arguments (Repeating)
        varargout
    end
end
```

### Incorrect

```matlab
function y = f(a, b)
    arguments
        b = 3                 % FVORDP: optional before required
        a (1,1) double
        opts.Name = 'x'       % FVORDN: name-value after positional (also FVAPN)
    end
    y = a;
end

function [y] = g(x)
    arguments (Output)
        y (1,1) double = 5    % FVOOD: default value for an output
    end
    arguments (Input)
        x (1,1) double        % FVOBI: input block after output block
    end
end
```

## Configuration

These checks are part of the `LANGUAGE_SPEC_ENGINE` rule. Disable individual checks via `disabled_checks`:

```toml title=".mlt.toml"
[lint.rules.LANGUAGE_SPEC_ENGINE]
disabled_checks = ["FVNIV", "TTOOFEWDIMS"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable within the language spec engine |

## Automatic fixes

No automatic fixes are available for these checks.

## Target node types

These checks run at the file level and inspect `function_definition` nodes and their `arguments_statement` children:

- `function_definition` — matched to extracted function metadata by byte range
- `arguments_statement` — an `arguments ... end` block, grouped with its attributes and arguments
- `attributes` — the block's `(Input)`, `(Output)`, `(Repeating)` attributes
- `property` — a single argument declaration
- `property_name` — a name-value argument declaration (`struct.field`)
- `class_property` — a `.?ClassName` name-value declaration
- `dimensions` — the size constraint
- `validation_functions` — the `{...}` validation function list
- `default_value` — the `= value` default expression

## Related rules

- `FVNST` — Arguments blocks in nested functions
- `VTPOD` — Argument validation order (size, then class, then functions)
- `FVINR` — Add Input attribute to the input arguments block (readability)

---

# Language Specification - Parfor Variable, Slicing, and Reduction Checks

These checks are also part of the `LANGUAGE_SPEC_ENGINE` rule. They validate parfor loop headers, banned functions, sliced output variables, and temporary/reduction variable classification, matching MATLAB Code Analyzer's parfor checks (PF\* family plus FPFORP and FWFORP).

## Check IDs

### Banned functions in parfor

| Check ID | Message |
| -------- | ------- |
| `PFEVC` | EVALIN('caller') and ASSIGNIN('caller') are invalid inside a PARFOR loop. |
| `PFINPT` | 'inputname' is not supported in parfor loops. |
| `PFNACK` | 'narginchk' and 'nargoutchk' cannot be used in parfor loops. |
| `PFNAIO` | 'nargin' and 'nargout' require a function argument in parfor loops. |
| `PFLD` | 'load' must assign to an output variable in parfor loops. |
| `PFSV` | SAVE cannot be called in a PARFOR loop without the '-fromstruct' option. |
| `FPFORP` | 'fprintf' is writing to a file opened with read permission only. |
| `FWFORP` | 'fwrite' is writing to a file opened with read permission only. |
| `PFCEL` | The function does not support cell arrays (argument N). |

### Parfor loop variable restrictions

| Check ID | Message |
| -------- | ------- |
| `PFANSLP` | 'ans' is not supported as a parfor loop variable. |
| `PFANSNS` | 'ans' is not supported as a for loop variable in parfor loops. |
| `PFVSUB` | Indexing parfor loop variables is not supported in parfor loops. |
| `PFFSUB` | Indexing a nested for loop variable is not supported in parfor loops. |
| `PFMLTI` | The nested for loop variable must not be assigned other than by its for statement. |
| `PFRNG` | The range of a PARFOR statement must be increasing consecutive integers. |

### Sliced variable rules

| Check ID | Message |
| -------- | ------- |
| `PFSLO` | Variable is indexed using the parfor loop variable, but it is not a valid sliced output variable. |
| `PFSLRD` | Invalid combination of sliced indexing and non-indexed reads of a sliced output variable. |
| `PFSLW` | Multiple sliced accesses must all use the same list of subscripts. |
| `PFANON` | Using a sliced output variable in an anonymous function is not supported in parfor loops. |
| `PFCTXT` | When indexing a sliced variable with a nested for loop variable, the sliced variable must be inside the for loop that defines its range. |
| `PFFRNG` | When indexing a sliced variable with a nested for loop variable, the range must be a row vector of positive constant numbers. |

### Temporary and reduction variables

| Check ID | Message |
| -------- | ------- |
| `PFUTMP` | Temporary variable must be set inside the parfor loop before it is used. |
| `PFUTVR` | Variable may have been intended as a reduction variable, but is an uninitialized temporary. |
| `PFINCR` | Using different reduction functions with the same reduction variable is not supported in parfor loops. |
| `PFNAR` | Subtracting reduction variable from expressions is not supported in parfor loops. |
| `PFRFH` | The PARFOR reduction function must be a function name or a broadcast variable. |
| `PFUNK` | The PARFOR loop cannot run due to the way a variable is used. |
| `PFVARS` | Parfor loop contains too many variables. |

## Why these checks matter

- **Parfor transparency** requires that every iteration be independent; functions that inspect the caller workspace (`evalin`/`assignin` with `'caller'`, `inputname`, `nargin`/`nargout`) or mutate the base workspace (`load`, `save`) break this model.
- **Sliced output variables** must be indexed by the parfor loop variable with a consistent subscript list; violations either serialize the loop, produce race conditions, or are rejected by the parallel pool.
- **Temporary variables** must be assigned before first use in every iteration; uninitialized temporaries are a common source of nondeterministic results.
- **Reduction variables** support a fixed set of associative/commutative functions; mixing operators or subtracting the reduction variable from an expression is rejected.

## Examples

### Correct

```matlab
parfor i = 1:10
    x(i) = i;              % sliced output with consistent subscripts
end

s = 0;
parfor i = 1:10
    s = s + x(i);          % valid reduction (initialized before the loop)
end
```

### Incorrect

```matlab
parfor i = 1:10
    evalin('caller', 'x'); % PFEVC
    load('data.mat');      % PFLD
    t = i * 2;
    y = t(i);              % PFSLO: t is a temporary, not a sliced output
    s = x(i) - s;          % PFNAR: subtracting reduction variable s
end
```

## Configuration

These checks are part of the `LANGUAGE_SPEC_ENGINE` rule. Disable individual checks via `disabled_checks`:

```toml title=".mlt.toml"
[lint.rules.LANGUAGE_SPEC_ENGINE]
disabled_checks = ["PFEVC", "PFSLO"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable within the language spec engine |

## Automatic fixes

No automatic fixes are available for these checks.

## Target node types

These checks run at the file level and inspect the following tree-sitter nodes inside parfor bodies:

- `for_statement` — a `parfor` or nested `for` loop (distinguished by the `parfor`/`for` keyword child)
- `function_call` — banned functions, sliced/indexed accesses, and loop-variable indexing
- `assignment` — temporary, sliced, and reduction variable classification
- `command` — command-syntax `load`, `save`, `nargin`, `nargout`
- `identifier` — whole (non-indexed) reads
- `lambda` — anonymous functions referencing sliced output variables

## Related rules

- `PFPF` — Nested parfor inside parfor
- `PFFORA` — Assignment to a for-loop variable inside parfor
- `PFXST` — Assignment to the parfor loop variable
- `PFUIXE` — Parfor index used in eval (bugs engine)
- `PFRNC` — Reduction not consistent in parfor (bugs engine)

---

# Language Specification - Class/Method Attribute, Constructor, and Superclass Checks

These checks are also part of the `LANGUAGE_SPEC_ENGINE` rule. They validate class and member attribute combinations, Constant and WeakHandle property usage, constructor/superclass call patterns, and method signature limits, matching MATLAB Code Analyzer's class checks (MC\* family plus `MTAGS3`, `MTMAT`, `MWKCL`, `MWKCT`, `MWKREF`).

## Check IDs

### Class attribute conflicts

| Check ID | Message |
| -------- | ------- |
| `MABSEAC` | Instance properties and methods are illegal in classes that are both Sealed and Abstract. |
| `MCSWA` | A sealed class cannot specify allowed subclasses. |
| `MTMAT` | Attribute can only be set once. |
| `MTAGS3` | Cannot use the Access attribute when using the SetAccess or GetAccess attribute. |

### Abstract/Sealed members

| Check ID | Message |
| -------- | ------- |
| `MABSEAM` | A method cannot be both Abstract and Sealed. |
| `MCMSP` | Private method cannot be Abstract. |
| `MCAPP` | Private property cannot be Abstract. |
| `MCGSA` | Method tries to set or get an abstract property. |
| `MCPSG` | Set or get method must be fully defined in the class definition file. |

### Constant and WeakHandle properties

| Check ID | Message |
| -------- | ------- |
| `MCCSOP` | Unable to modify Constant property. |
| `MCSCN` | Method tries to set a constant property. |
| `MWKCL` | A WeakHandle property must restrict its type using a class validation. |
| `MWKCT` | Specifying both WeakHandle and Constant attributes on the same property is not supported. |
| `MWKREF` | Specifying both WeakHandle and Dependent attributes is invalid. |

### Constructor/superclass calls

| Check ID | Message |
| -------- | ------- |
| `MCCBS` | A superclass constructor is being called, but the name is not a declared superclass name. |
| `MCCBU` | This superclass constructor is called after a use of the constructed object. |
| `MCCMC` | Constructor for superclass can only be called once. |
| `MCSCC` | To call the superclass constructor, the subclass constructor name must match the subclass name. |
| `MCSCF` | A superclass constructor must be assigned to the first constructor output argument. |
| `MCSCO` | A superclass constructor must be called using the first constructor output argument. |
| `MCSCT` | Superclass constructor call must not be conditionalized or be part of another expression. |
| `MCSMO` | Returning multiple outputs from a superclass object initialization is not supported. |
| `MCSCM` | To call a superclass method, the method name must match the subclass method name. |

### Method signatures

| Check ID | Message |
| -------- | ------- |
| `MCMIO` | Method has too many inputs or outputs. |
| `MCMTP` | TestParameterDefinition methods must be Static. |
| `MCPIN` | Unable to initialize class property to an instance of the class itself. |

## Why these checks matter

- **Mutually exclusive attributes** (`Sealed`+`Abstract`, `WeakHandle`+`Constant`, `WeakHandle`+`Dependent`, `Access`+`SetAccess`) describe states MATLAB cannot represent; the class is rejected at definition time.
- **Abstract members** must not be private or sealed, and set/get methods must be fully implemented — abstract accessors leave the property unusable.
- **Constant properties** cannot be reassigned after class load; assignments in the constructor or other methods are silently lost or error at runtime.
- **Superclass constructor calls** (`obj@SuperClass(...)`) must be the first operation, unconditional, assigned to the first output, and reference a declared superclass — violations produce undefined objects or runtime errors.
- **WeakHandle properties** need a class validation so MATLAB knows the concrete type the handle refers to.

## Examples

### Correct

```matlab
classdef GoodClass < Base
    properties (Constant)
        K = 42
    end
    properties (WeakHandle)
        ref SomeHandleClass
    end
    methods
        function obj = GoodClass()
            obj = obj@Base();
        end
    end
end
```

### Incorrect

```matlab
classdef (Sealed, Abstract) BadClass
    properties
        x                          % MABSEAC: instance property in Sealed+Abstract class
    end
    properties (WeakHandle, Constant)
        w                          % MWKCT
    end
    methods (Abstract, Sealed)
        function y = f(obj)        % MABSEAM
        end
    end
    methods
        function obj = BadClass()
            obj.y = 1;             % MCCBU: object used before superclass constructor
            if true
                obj = obj@Base();  % MCSCT: conditionalized superclass constructor
            end
            [obj, z] = obj@Base(); % MCCMC + MCSMO: second call with multiple outputs
        end
    end
end
```

## Configuration

These checks are part of the `LANGUAGE_SPEC_ENGINE` rule. Disable individual checks via `disabled_checks`:

```toml title=".mlt.toml"
[lint.rules.LANGUAGE_SPEC_ENGINE]
disabled_checks = ["MABSEAC", "MCSWA"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable within the language spec engine |

## Automatic fixes

No automatic fixes are available for these checks.

## Target node types

These checks run at the file level and use both `FileMeta`/`ClassMeta` metadata and raw tree walks:

- `class_definition` — class-level attribute checks (MABSEAC, MCSWA, MTMAT, MTAGS3)
- `properties` — property attribute checks (MCAPP, MCGSA, MWKCL, MWKCT, MWKREF, MCPIN)
- `methods` — method attribute checks (MABSEAM, MCMSP, MCPSG, MCMTP, MCMIO)
- `function_definition` — constructor/superclass call checks (MCCBS, MCCBU, MCCMC, MCSCF, MCSCN, MCSCO, MCSCT, MCSMO, MCSCC, MCSCM, MCCSOP)
- `assignment` — `obj.Prop = value` assignments to Constant properties

## Related rules

- `MCFIL`, `MCDIR`, `MCRED`, `MCCBD` — class file/name structure checks
- `MCS2I`, `MCS1O`, `MCG1I`, `MCG1O`, `MCSGP` — set/get method signature checks
- `CTOINE`, `CTORO`, `MHERIT` — constructor construction checks in this engine
