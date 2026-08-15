---
icon: lucide/sliders-horizontal
---

# Custom Checks

**Default severity:** Warning
**Auto-fix:** No
**Category:** Custom Checks
**Can be disabled:** Yes

## What this rule does

Configurable code-complexity and style-metric checks from MATLAB's Code
Analyzer. All 25 checks are handled by a single file-level engine
(`CustomChecksEngine`) that performs one traversal per file and computes
all metrics simultaneously. Each check has a configurable threshold; a
threshold of `0` disables that check.

## Check IDs

### SYSBANG

Severity: **warning** · Auto-fix: **no**

System command used (!)

### FCNIL

Severity: **warning** · Auto-fix: **no**

Function input count exceeds limit

### FCNOL

Severity: **warning** · Auto-fix: **no**

Function output count exceeds limit

### FCNLL

Severity: **warning** · Auto-fix: **no**

Function line count exceeds limit

### LLMNC

Severity: **warning** · Auto-fix: **no**

Line length exceeds limit

### MNCSN

Severity: **warning** · Auto-fix: **no**

Statement nesting depth exceeds limit

### DAFTC

Severity: **warning** · Auto-fix: **no**

Too many children in tree

### DAFPV

Severity: **warning** · Auto-fix: **no**

Too many persistent variables

### DAFCO

Severity: **warning** · Auto-fix: **no**

Too many conditions in expression

### DAFBR

Severity: **warning** · Auto-fix: **no**

Too many branches in switch/if

### DAFRT

Severity: **warning** · Auto-fix: **no**

Too many return points

### DAFSC

Severity: **warning** · Auto-fix: **no**

Too many semicolons on one line

### DAFNF

Severity: **warning** · Auto-fix: **no**

Too many nested functions

### DAFCF

Severity: **warning** · Auto-fix: **no**

Too many called functions

### DAFAF

Severity: **warning** · Auto-fix: **no**

Too many anonymous functions

### DAFCV

Severity: **warning** · Auto-fix: **no**

Too many local variables

### DAFCVC

Severity: **warning** · Auto-fix: **no**

Too many local constants

### DAFVI

Severity: **warning** · Auto-fix: **no**

Too many input arguments used

### DAFVO

Severity: **warning** · Auto-fix: **no**

Too many output arguments used

### CYCCOM

Severity: **warning** · Auto-fix: **no**

Cyclomatic complexity of function exceeds limit

### SCYCCOM

Severity: **warning** · Auto-fix: **no**

Strict cyclomatic complexity exceeds limit

### ACYCCOM

Severity: **warning** · Auto-fix: **no**

Average cyclomatic complexity exceeds limit

### MCYCCOM

Severity: **warning** · Auto-fix: **no**

Method cyclomatic complexity exceeds limit

### MSCYCCOM

Severity: **warning** · Auto-fix: **no**

Method strict cyclomatic complexity exceeds limit

### MACYCCOM

Severity: **warning** · Auto-fix: **no**

Method average cyclomatic complexity exceeds limit

## Examples

### Incorrect

```matlab
function f(a, b, c, d, e, f, g, h, i)  % FCNIL — too many inputs
    x = 1; y = 2; z = 3;               % DAFSC — too many semicolons on one line
end
```

### Correct

```matlab
function f(a, b, c)  % within the default input limit
    x = 1;
end
```

## Configuration

```toml
[lint.rules.CUSTOM_CHECKS]
severity = "warn"
max_function_inputs = 7
max_function_outputs = 7
max_function_lines = 200
max_line_length = 120
max_nesting_depth = 5
max_cyclomatic_complexity = 15
max_strict_cyclomatic_complexity = 20
max_avg_cyclomatic_complexity = 10
max_branches = 10
max_return_points = 5
max_nested_functions = 3
max_anonymous_functions = 5
max_local_variables = 20
max_local_constants = 10
max_called_functions = 30
max_semicolons_per_line = 3
max_tree_children = 50
max_persistent_variables = 10
max_conditions = 5
max_input_args_used = 10
max_output_args_used = 10
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
