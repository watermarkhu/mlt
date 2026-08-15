---
icon: lucide/book-marked
---

# Language Specification Errors

**Default severity:** Error
**Auto-fix:** No
**Category:** Language Specification
**Can be disabled:** Yes

## What this rule does

Enforces MATLAB language-specification constraints from the Code Analyzer.
All 165 checks share a single file-level engine (`LanguageSpecEngine`) that
performs a full DFS traversal with context stacks for parfor/spmd/loop
detection, then runs class, arguments-block, global/persistent, and
script-level checks over extracted file metadata. Each diagnostic carries
the specific check ID (e.g. `PFPF`, `FVAPN`, `MCFIL`).

The checks are grouped into families:

- **PF\*** — parfor restrictions, banned functions, and sliced-variable /
  reduction classification.
- **SP\*** — spmd block restrictions (`SPNST`, `SPRET`, `SPGP`, and the
  transparency group `SPDEC`, `SPEVC`, `SPLD`, `SPNF`, `SPSV`, `SPWHOS`,
  `SPBFN`).
- **AT\*** — class, events, property, and method attribute validation.
- **CLS\* / NOPRV** — class-file structure checks.
- **VTP\*** — property validation function checks.
- **MC\*** — class/OOP rules: naming, set/get signatures, constructor and
  superclass calls, Constant/WeakHandle properties.
- **FV\*** — `arguments` block validation (plus `TINVALDIM`,
  `TTOOFEWDIMS`).
- **Miscellaneous** — `FCONV`, `FCONF`, `GPFST`, `GPNES`, `NPERS`,
  `ROWLN`, `FCNANS`, `CLANS`, `BRKFOR`, `CONTFOR`, `IDXCOLND`, `ERTXT`,
  `WTXT`, `MHERIT`, `NCHKOS`, `CTOINE`, `CTORO`, `USESWNS`.

## Check IDs

### PFPF

Severity: **error** · Auto-fix: **no**

Nested parfor inside parfor

### PFSPMD

Severity: **error** · Auto-fix: **no**

SPMD inside parfor

### PFBRK

Severity: **error** · Auto-fix: **no**

break inside parfor

### PFRTN

Severity: **error** · Auto-fix: **no**

return inside parfor

### PFGLOB

Severity: **error** · Auto-fix: **no**

global declaration inside parfor

### PFPERS

Severity: **error** · Auto-fix: **no**

persistent declaration inside parfor

### PFFORA

Severity: **error** · Auto-fix: **no**

Assignment to for-loop variable inside parfor

### PFXST

Severity: **error** · Auto-fix: **no**

Assignment to parfor loop variable

### PFNF

Severity: **error** · Auto-fix: **no**

Nested function call inside parfor

### FPFORP

Severity: **error** · Auto-fix: **no**

fprintf writing to a file opened read-only

### FWFORP

Severity: **error** · Auto-fix: **no**

fwrite writing to a file opened read-only

### PFANON

Severity: **error** · Auto-fix: **no**

Sliced output variable used in an anonymous function

### PFANSLP

Severity: **error** · Auto-fix: **no**

'ans' as parfor loop variable

### PFANSNS

Severity: **error** · Auto-fix: **no**

'ans' as for loop variable inside parfor

### PFCEL

Severity: **error** · Auto-fix: **no**

Function does not support cell arrays

### PFCTXT

Severity: **error** · Auto-fix: **no**

Sliced variable indexed outside its defining for loop

### PFEVC

Severity: **error** · Auto-fix: **no**

EVALIN/ASSIGNIN('caller') invalid inside parfor

### PFFRNG

Severity: **error** · Auto-fix: **no**

Nested for range must be positive constants

### PFFSUB

Severity: **error** · Auto-fix: **no**

Indexing a nested for loop variable

### PFINCR

Severity: **error** · Auto-fix: **no**

Different reduction functions on the same variable

### PFINPT

Severity: **error** · Auto-fix: **no**

'inputname' not supported in parfor

### PFLD

Severity: **error** · Auto-fix: **no**

'load' must assign to an output in parfor

### PFMLTI

Severity: **error** · Auto-fix: **no**

Nested for loop variable assigned in the body

### PFNACK

Severity: **error** · Auto-fix: **no**

narginchk/nargoutchk cannot be used in parfor

### PFNAIO

Severity: **error** · Auto-fix: **no**

nargin/nargout require a function argument

### PFNAR

Severity: **error** · Auto-fix: **no**

Subtracting a reduction variable from expressions

### PFRFH

Severity: **error** · Auto-fix: **no**

Reduction function must be a name or broadcast variable

### PFRNG

Severity: **error** · Auto-fix: **no**

Parfor range must be increasing consecutive integers

### PFSLO

Severity: **error** · Auto-fix: **no**

Variable indexed with parfor var is not a sliced output

### PFSLRD

Severity: **error** · Auto-fix: **no**

Sliced indexing combined with non-indexed reads

### PFSLW

Severity: **error** · Auto-fix: **no**

Sliced accesses must all use the same subscripts

### PFSV

Severity: **error** · Auto-fix: **no**

SAVE requires '-fromstruct' in parfor

### PFUNK

Severity: **error** · Auto-fix: **no**

Parfor cannot run due to the way a variable is used

### PFUTMP

Severity: **error** · Auto-fix: **no**

Temporary variable must be set before it is used

### PFUTVR

Severity: **error** · Auto-fix: **no**

Variable intended as reduction but uninitialized

### PFVARS

Severity: **error** · Auto-fix: **no**

Parfor loop contains too many variables

### PFVSUB

Severity: **error** · Auto-fix: **no**

Indexing the parfor loop variable

### PFANSRE

Severity: **error** · Auto-fix: **no**

'ans' is not supported as a reduction variable

### PFANSSL

Severity: **error** · Auto-fix: **no**

'ans' is not supported as a sliced variable

### PFDF

Severity: **error** · Auto-fix: **no**

FOR with DRANGE becomes a conventional FOR inside a PARFOR

### PFPIE

Severity: **error** · Auto-fix: **no**

Valid indices for a sliced variable are restricted

### PFSAME

Severity: **error** · Auto-fix: **no**

Sliced variable indexed in different ways

### PFTIN

Severity: **error** · Auto-fix: **no**

Temporary variable must be set before it is used

### ATUNK

Severity: **error** · Auto-fix: **no**

Unknown attribute name

### ATLAB

Severity: **error** · Auto-fix: **no**

'Input'/'Output' attribute must not be valued or negated

### ATNAS

Severity: **error** · Auto-fix: **no**

Meta-class attribute must be a meta-class or cell array

### ATNPI

Severity: **error** · Auto-fix: **no**

Class access attribute has an unexpected value

### ATNPP

Severity: **error** · Auto-fix: **no**

Events access attribute has an unexpected value

### ATPPI

Severity: **error** · Auto-fix: **no**

Property access attribute has an unexpected value

### ATPPP

Severity: **error** · Auto-fix: **no**

Method access attribute has an unexpected value

### ATAS

Severity: **error** · Auto-fix: **no**

Meta-class attribute value is unexpected

### ATVIZE

Severity: **error** · Auto-fix: **no**

'Visible' attribute is invalid for classes/events

### CLSAT

Severity: **error** · Auto-fix: **no**

Specify class attributes before the class name

### CLSUNK

Severity: **error** · Auto-fix: **no**

Class or superclass could not be found on the path

### NOPRV

Severity: **error** · Auto-fix: **no**

Class definition cannot be inside a private directory

### VTPCON

Severity: **error** · Auto-fix: **no**

Validation functions must only use the property or literals

### VTPEAL

Severity: **error** · Auto-fix: **no**

Specify at least one input argument for validator

### VTPIN

Severity: **error** · Auto-fix: **no**

Validation function must use the property as an input

### SPNST

Severity: **error** · Auto-fix: **no**

parfor or spmd inside spmd

### SPRET

Severity: **error** · Auto-fix: **no**

return/break/continue inside spmd

### SPGP

Severity: **error** · Auto-fix: **no**

global/persistent inside spmd

### MCFIL

Severity: **error** · Auto-fix: **no**

Class name and file name don't match

### MCDIR

Severity: **error** · Auto-fix: **no**

Class name and @directory name don't match

### MCRED

Severity: **error** · Auto-fix: **no**

Property/event/enum name same as class name

### MCCBD

Severity: **error** · Auto-fix: **no**

Constructor not in class definition file

### MCS2I

Severity: **error** · Auto-fix: **no**

Setter must have exactly 2 inputs

### MCS1O

Severity: **error** · Auto-fix: **no**

Setter must have at most 1 output

### MCG1I

Severity: **error** · Auto-fix: **no**

Getter must have exactly 1 input

### MCG1O

Severity: **error** · Auto-fix: **no**

Getter must have exactly 1 output

### MCEB

Severity: **error** · Auto-fix: **no**

Events defined in non-handle class

### MCANI

Severity: **error** · Auto-fix: **no**

Abstract property initialized

### MCASC

Severity: **error** · Auto-fix: **no**

Abstract property in Sealed class

### MCSGA

Severity: **error** · Auto-fix: **no**

Set/get method in methods block with attributes

### MCSGP

Severity: **error** · Auto-fix: **no**

Set/get method refers to invalid property

### MABSEAC

Severity: **error** · Auto-fix: **no**

Instance properties/methods illegal in Sealed+Abstract classes

### MABSEAM

Severity: **error** · Auto-fix: **no**

A method cannot be both Abstract and Sealed

### MCAPP

Severity: **error** · Auto-fix: **no**

Private property cannot be Abstract

### MCCBS

Severity: **error** · Auto-fix: **no**

Superclass constructor is not a declared superclass name

### MCCBU

Severity: **error** · Auto-fix: **no**

Superclass constructor called after object use

### MCCMC

Severity: **error** · Auto-fix: **no**

Constructor for superclass can only be called once

### MCCSOP

Severity: **error** · Auto-fix: **no**

Unable to modify Constant property

### MCGSA

Severity: **error** · Auto-fix: **no**

Set/get method tries to access an abstract property

### MCMIO

Severity: **error** · Auto-fix: **no**

Method has too many inputs or outputs

### MCMSP

Severity: **error** · Auto-fix: **no**

Private method cannot be Abstract

### MCMTP

Severity: **error** · Auto-fix: **no**

TestParameterDefinition methods must be Static

### MCPIN

Severity: **error** · Auto-fix: **no**

Property initialized to instance of the class itself

### MCPSG

Severity: **error** · Auto-fix: **no**

Set or get method must be fully defined in the class file

### MCSCC

Severity: **error** · Auto-fix: **no**

Superclass constructor call needs a matching subclass constructor name

### MCSCF

Severity: **error** · Auto-fix: **no**

Superclass constructor must be assigned to the first output

### MCSCM

Severity: **error** · Auto-fix: **no**

Superclass method call needs a matching method name

### MCSCN

Severity: **error** · Auto-fix: **no**

Method tries to set a constant property

### MCSCO

Severity: **error** · Auto-fix: **no**

Superclass constructor must use the first output argument

### MCSCT

Severity: **error** · Auto-fix: **no**

Superclass constructor call must not be conditionalized

### MCSMO

Severity: **error** · Auto-fix: **no**

Multiple outputs from superclass initialization unsupported

### MCSWA

Severity: **error** · Auto-fix: **no**

Sealed class cannot specify allowed subclasses

### MTAGS3

Severity: **error** · Auto-fix: **no**

Access attribute conflicts with SetAccess/GetAccess

### MTMAT

Severity: **error** · Auto-fix: **no**

Attribute can only be set once

### MWKCL

Severity: **error** · Auto-fix: **no**

WeakHandle property must have a class validation

### MWKCT

Severity: **error** · Auto-fix: **no**

WeakHandle and Constant attributes conflict

### MWKREF

Severity: **error** · Auto-fix: **no**

WeakHandle and Dependent attributes conflict

### FVNST

Severity: **error** · Auto-fix: **no**

Arguments blocks in nested functions

### FVAPN

Severity: **error** · Auto-fix: **no**

Move name=value name-value arguments to the end

### FVATF

Severity: **error** · Auto-fix: **no**

Attribute values in arguments blocks must be logical constants

### FVBTN

Severity: **error** · Auto-fix: **no**

Use of this function is not supported in arguments blocks

### FVDAN

Severity: **error** · Auto-fix: **no**

Same name as name-value structure and positional

### FVDAP

Severity: **error** · Auto-fix: **no**

Positional argument can only be declared once

### FVDNF

Severity: **error** · Auto-fix: **no**

Name-value argument can only be declared once

### FVDREP

Severity: **error** · Auto-fix: **no**

Multiple Repeating arguments blocks not supported

### FVIDV

Severity: **error** · Auto-fix: **no**

Validation/default on ignored arguments unsupported

### FVIOA

Severity: **error** · Auto-fix: **no**

Both 'Input' and 'Output' attributes on one block

### FVMCL

Severity: **error** · Auto-fix: **no**

Multiple name-value structures using .? syntax

### FVNDE

Severity: **error** · Auto-fix: **no**

Default values illegal for class-name name-value

### FVNIV

Severity: **error** · Auto-fix: **no**

Variable is not an input to the function

### FVNREP

Severity: **error** · Auto-fix: **no**

Name-value arguments in Repeating block unsupported

### FVNSC

Severity: **error** · Auto-fix: **no**

Calling nested functions in arguments blocks

### FVNVL

Severity: **error** · Auto-fix: **no**

Validation illegal for class-name name-value

### FVOBI

Severity: **error** · Auto-fix: **no**

Declare input blocks before output blocks

### FVOCON

Severity: **error** · Auto-fix: **no**

Output validation only uses arg or literals

### FVOND

Severity: **error** · Auto-fix: **no**

Name-value arguments in default values unsupported

### FVONV

Severity: **error** · Auto-fix: **no**

Name-value arguments without dotted name in validation

### FVOOD

Severity: **error** · Auto-fix: **no**

Default value for output argument unsupported

### FVOOI

Severity: **error** · Auto-fix: **no**

Ignored arguments in output block unsupported

### FVOON

Severity: **error** · Auto-fix: **no**

Name-value argument as output unsupported

### FVORDI

Severity: **error** · Auto-fix: **no**

Ignored inputs after Repeating or name-value

### FVORDN

Severity: **error** · Auto-fix: **no**

Positional arguments before name-value arguments

### FVORDO

Severity: **error** · Auto-fix: **no**

Repeating outputs after required outputs

### FVORDP

Severity: **error** · Auto-fix: **no**

Positional order: required, optional, repeating

### FVORM

Severity: **error** · Auto-fix: **no**

Multiple repeating output arguments unsupported

### FVOVREP

Severity: **error** · Auto-fix: **no**

varargout only in Repeating output block

### FVREPD

Severity: **error** · Auto-fix: **no**

Defaults in Repeating block unsupported

### FVREPO

Severity: **error** · Auto-fix: **no**

Repeating input block with varargin has no other args

### FVSOR

Severity: **error** · Auto-fix: **no**

Input block matches function line order

### FVSORO

Severity: **error** · Auto-fix: **no**

Output block matches function line order

### FVUBD

Severity: **error** · Auto-fix: **no**

Argument referenced before declared

### FVVCON

Severity: **error** · Auto-fix: **no**

Input validation only uses prior positionals

### FVVIN

Severity: **error** · Auto-fix: **no**

Validation function must use the argument

### FVVREP

Severity: **error** · Auto-fix: **no**

varargin only inside repeating input block

### TINVALDIM

Severity: **error** · Auto-fix: **no**

Each dimension must be nonnegative integer or colon

### TTOOFEWDIMS

Severity: **error** · Auto-fix: **no**

Specify at least two dimensions for size

### FCONV

Severity: **error** · Auto-fix: **no**

Variable name same as script name

### FCONF

Severity: **error** · Auto-fix: **no**

Local function name same as file name

### GPFST

Severity: **error** · Auto-fix: **no**

Global/persistent must precede first use

### GPNES

Severity: **error** · Auto-fix: **no**

Global/persistent must be in outermost function

### NPERS

Severity: **error** · Auto-fix: **no**

Persistent in script

### ROWLN

Severity: **error** · Auto-fix: **no**

Matrix rows must be same length

### FCNANS

Severity: **error** · Auto-fix: **no**

Function named 'ans'

### CLANS

Severity: **error** · Auto-fix: **no**

Class named 'ans'

### BRKFOR

Severity: **error** · Auto-fix: **no**

break outside loop

### CONTFOR

Severity: **error** · Auto-fix: **no**

continue outside loop

### IDXCOLND

Severity: **error** · Auto-fix: **no**

END operator outside index expression

### CTOINE

Severity: **error** · Auto-fix: **no**

Use of constructed object as input to constructor is not supported

### CTORO

Severity: **error** · Auto-fix: **no**

Class constructors must be declared with at least one output argument

### ERTXT

Severity: **error** · Auto-fix: **no**

Specify an error message with the message identifier

### MHERIT

Severity: **error** · Auto-fix: **no**

Deriving from a built-in MATLAB class is not supported

### NCHKOS

Severity: **error** · Auto-fix: **no**

NARGINCHK does not return any values

### SPBFN

Severity: **error** · Auto-fix: **no**

Non-transparent workspace access inside spmd

### SPDEC

Severity: **error** · Auto-fix: **no**

SPMD worker bounds must be nonnegative integers

### SPDEC3

Severity: **error** · Auto-fix: **no**

SPMD can only specify lower and upper worker bounds

### SPEVC

Severity: **error** · Auto-fix: **no**

EVALIN/ASSIGNIN('caller') invalid inside spmd

### SPLD

Severity: **error** · Auto-fix: **no**

Assign the output of LOAD in spmd blocks

### SPNF

Severity: **error** · Auto-fix: **no**

Nested function call inside spmd

### SPSV

Severity: **error** · Auto-fix: **no**

SAVE requires '-fromstruct' inside spmd

### SPWHOS

Severity: **error** · Auto-fix: **no**

who/whos without '-file' invalid inside spmd

### USESWNS

Severity: **error** · Auto-fix: **no**

Variable must be explicitly defined before first use

### WTXT

Severity: **error** · Auto-fix: **no**

Specify a warning message with the message identifier

## Examples

### Incorrect

```matlab
parfor i = 1:10          % PFPF: nested parfor
    parfor j = 1:10
    end
    break                % PFBRK: break inside parfor
end

function y = f(a)
    arguments (Input, Output)   % FVIOA
        a (1,1) double = 1      % FVOOD: default on output
    end
end
```

### Correct

```matlab
parfor i = 1:10
    x(i) = i;
end

function y = f(a)
    arguments (Input)
        a (1,1) double
    end
    arguments (Output)
        y (1,1) double
    end
    y = a;
end
```

## Configuration

```toml
[lint.rules.LANGUAGE_SPEC_ENGINE]
severity = "error"
disabled_checks = ["PFPF", "FVIOA"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
