---
icon: lucide/package
---

# MATLAB Compiler Deployment Constraint Checks

**Default severity:** Warning
**Auto-fix:** No
**Category:** Deployment
**Can be disabled:** Yes

## What this rule does

Detects MATLAB constructs that are problematic in compiled, deployed
applications built with `mcc`. Deployed MATLAB applications run in a
restricted MCR environment where certain functions are unavailable or
behave differently. All 10 checks share a single `DeploymentEngine` that
dispatches node-level checks on `function_call` and `command` nodes; each
diagnostic carries the specific check ID (e.g. `MCCD`, `MCTBX`) and its
own severity.

## Check IDs

### MCCD

Severity: **error** · Auto-fix: **no**

'cd' should not be used in deployed applications

### MCPRD

Severity: **error** · Auto-fix: **no**

Path modification functions should not be used in deployed applications

### MCHLP

Severity: **warning** · Auto-fix: **no**

'help'/'doc' are not available in deployed applications

### MCKBD

Severity: **warning** · Auto-fix: **no**

'keyboard' is not available in deployed applications

### MCSVP

Severity: **warning** · Auto-fix: **no**

'savepath' is not available in deployed applications

### MCMLR

Severity: **warning** · Auto-fix: **no**

'matlabroot' returns the MCR root, not the MATLAB root

### MCABF

Severity: **error** · Auto-fix: **no**

'addpath' with an absolute path will fail in deployed applications

### MCMFL

Severity: **warning** · Auto-fix: **no**

'mfilename' behaves differently in deployed applications

### MCTBX

Severity: **warning** · Auto-fix: **no**

Toolbox function may not be available without proper toolbox compilation

### MCLL

Severity: **error** · Auto-fix: **no**

License checking is not available in deployed applications

## Examples

### Incorrect

```matlab
cd /tmp;              % MCCD
addpath('/abs/path'); % MCABF
doc plot;             % MCHLP
```

### Correct

```matlab
% Bundle resources with the deployed app and use relative paths.
result = processFile('data.bin');
```

## Configuration

```toml
[lint.rules.DEPLOYMENT_ENGINE]
severity = "warning"
skip_checks = ["MCTBX"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
