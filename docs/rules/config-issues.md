---
icon: lucide/cog
---

# Configuration Issues

**Default severity:** Error
**Auto-fix:** No
**Category:** Configuration Issues
**Can be disabled:** Yes

## What this rule does

Detects MATLAB-side configuration issues such as invalid parameter names
passed to configuration functions, wrong argument counts, and invalid
option values. The engine matches `function_call` and `command` nodes
against known configuration function patterns and emits a diagnostic with
the specific check ID (`BDCFG`, `CFERR`, `BDOPT`, `CFIG`).

## Check IDs

### BDCFG

Severity: **error** · Auto-fix: **no**

Invalid configuration parameter

### CFERR

Severity: **error** · Auto-fix: **no**

Configuration function error

### BDOPT

Severity: **error** · Auto-fix: **no**

Invalid option value

### CFIG

Severity: **error** · Auto-fix: **no**

Configuration file issue

## Examples

### Incorrect

```matlab
set_param(gcs, 'NotARealParam', 'value');
cfg = coder.config('something');
opts = optimset('NotAnOption', 1);
```

### Correct

```matlab
set_param(gcs, 'SimulationCommand', 'start');
cfg = coder.config('lib');
opts = optimset('Display', 'off');
```

## Configuration

```toml
[lint.rules.CONFIG_ISSUES_ENGINE]
disabled_checks = []
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
