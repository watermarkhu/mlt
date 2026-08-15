---
icon: lucide/boxes
---

# System Object Validation Checks

**Default severity:** Warning
**Auto-fix:** No
**Category:** System Objects
**Can be disabled:** Yes

## What this rule does

Validates MATLAB System object usage. System objects (classes inheriting
from `matlab.System`) have specific lifecycle constraints that static
analysis can partially verify. Since full type information is not available
to a static linter, these checks use pattern-based heuristics — detecting
System object patterns by class inheritance, lifecycle method calls
(`step`, `setup`, `release`, `reset`), and property access conventions.
All 9 checks share a single `SystemObjectsEngine` that dispatches node-level
checks on `function_call` nodes plus file-level class analysis; each
diagnostic carries the specific check ID (e.g. `SONUMIN`, `SOTUNPROP3`).

## Check IDs

### SONUMIN

Severity: **error** · Auto-fix: **no**

System object method called with wrong number of inputs

### SONUMOUT

Severity: **error** · Auto-fix: **no**

System object method called with wrong number of outputs

### SODEPPROP

Severity: **warning** · Auto-fix: **no**

Deprecated system object property; use the recommended replacement

### SOINITPROP

Severity: **warning** · Auto-fix: **no**

DiscreteState properties must be initialized within a 'resetImpl' method

### SODFLTVAL

Severity: **error** · Auto-fix: **no**

Property default value uses a function call, which may not be valid

### SORSRVDNM

Severity: **warning** · Auto-fix: **no**

Reserved name used for system object member; choose a different name

### SOTUNPROP1

Severity: **warning** · Auto-fix: **no**

Logical attribute not supported for tunable properties on MATLAB System blocks

### SOTUNPROP3

Severity: **warning** · Auto-fix: **no**

Tunable properties on System blocks must be numeric; char property is made Nontunable

### SOTUNPROP4

Severity: **warning** · Auto-fix: **no**

Tunable properties on System blocks must be numeric; string property is made Nontunable

## Examples

### Incorrect

```matlab
classdef MySystem < matlab.System
    properties
        Gain = rand();      % SODFLTVAL: function-call default value
        Flag logical = false % SOTUNPROP1: logical tunable property
    end
    methods
        function step(obj)   % SORSRVDNM: reserved method name
        end
    end
end
step(); % SONUMIN: step() called without inputs
```

### Correct

```matlab
classdef MySystem < matlab.System
    properties
        Gain = 1;
        Flag = false;
    end
    methods
        function stepImpl(obj) % use the *Impl override, not 'step'
        end
    end
end
step(obj, input); % pass the object and the input signal
```

## Configuration

```toml
[lint.rules.SYSTEM_OBJECTS_ENGINE]
severity = "warning"
skip_checks = ["SONUMIN"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
