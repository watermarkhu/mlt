---
icon: lucide/braces
---

# MATLAB Code Generation Constraint Checks

**Default severity:** Error
**Auto-fix:** No
**Category:** Code Generation
**Can be disabled:** Yes

## What this rule does

Detects MATLAB constructs that are unsupported or problematic for MATLAB
Coder / code generation. MATLAB code intended to be compiled to C/C++ has
many restrictions compared to general MATLAB code; this engine flags
variable-size data, growing arrays, unsupported functions, cell arrays,
try-catch, imports, nested functions, scripts, and similar constraints.
All 15 checks share a single `CodegenEngine` that dispatches node-level
checks on `function_call`, `command`, `try_statement`, `for_statement`,
`assignment`, `cell`, and `arguments_statement` nodes, plus file-level
checks for nested functions and script-mode files. Each diagnostic carries
the specific check ID (e.g. `EMFCN`, `EMSCR`).

## Check IDs

### EMVDF

Severity: **error** · Auto-fix: **no**

Variable-size data is not supported for code generation

### EMGRO

Severity: **error** · Auto-fix: **no**

Growing arrays inside loops is not supported for code generation

### EMFCN

Severity: **error** · Auto-fix: **no**

Function is not supported for code generation

### EMCEL

Severity: **error** · Auto-fix: **no**

Cell arrays are not supported for code generation

### EMTC

Severity: **error** · Auto-fix: **no**

Try-catch statements are not supported for code generation

### EMIMP

Severity: **error** · Auto-fix: **no**

Import statements are not supported for code generation

### EMNST

Severity: **error** · Auto-fix: **no**

Nested functions are not supported for code generation

### EMSCR

Severity: **error** · Auto-fix: **no**

Scripts are not supported; use functions instead

### EMPFR

Severity: **error** · Auto-fix: **no**

Parfor is not supported for code generation

### EMRIFAV

Severity: **error** · Auto-fix: **no**

Arguments validation block is not fully supported for code generation

### EMLOAD

Severity: **error** · Auto-fix: **no**

'load' is not supported for code generation

### EMS2N

Severity: **error** · Auto-fix: **no**

'str2num' is not supported; use 'str2double'

### PRMNOIN

Severity: **error** · Auto-fix: **no**

No input validation available in generated code

### LOOPPRAGMAWITHOUTFOR

Severity: **error** · Auto-fix: **no**

coder.loop pragma must be immediately followed by a for-loop

### FPASE

Severity: **error** · Auto-fix: **no**

Assignment to a scaled fixed-point expression may lose precision

## Examples

### Incorrect

```matlab
x = [];
for i = 1:10
    x(end+1) = i;   % EMGRO: growing array
end
y = {1, 2};         % EMCEL: cell array
z = str2num('1 2'); % EMS2N: use str2double
```

### Correct

```matlab
x = zeros(1, 10);
for i = 1:10
    x(i) = i;
end
y = [1, 2];
z = str2double('1 2');
```

## Configuration

```toml
[lint.rules.CODEGEN_ENGINE]
severity = "error"
skip_checks = ["EMSCR", "EMFCN"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
