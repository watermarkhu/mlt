---
icon: lucide/ban
---

# Unsupported Feature Checks

**Default severity:** Warning
**Auto-fix:** No
**Category:** Unsupported
**Can be disabled:** Yes

## What this rule does

Detects usage of unsupported, deprecated, or platform-specific features
that are no longer available in modern MATLAB or restricted to specific
configurations. All 13 checks share a single `UnsupportedEngine` that
dispatches node-level checks on `function_call` and `command` nodes; each
diagnostic carries the specific check ID (e.g. `MCADE`, `FEATUD`).

## Check IDs

### MCADE

Severity: **warning** · Auto-fix: **no**

ADE (Application Deployment Environment) function is no longer supported

### AWTIUD

Severity: **warning** · Auto-fix: **no**

Await syntax is not supported in this context

### AXCHUD

Severity: **warning** · Auto-fix: **no**

ActiveX/COM automation is deprecated; use modern alternatives

### FEATUD

Severity: **warning** · Auto-fix: **no**

'feature' is an undocumented internal function; avoid in production code

### FNDPUD

Severity: **warning** · Auto-fix: **no**

'findprop' is deprecated; use 'findobj' or property access instead

### HGCNUD

Severity: **warning** · Auto-fix: **no**

Handle Graphics container object pattern is deprecated

### IMPKG

Severity: **warning** · Auto-fix: **no**

Import package syntax is not supported in this context

### ISMBUD

Severity: **warning** · Auto-fix: **no**

'isMember' (camelCase) is deprecated; use 'ismember' (lowercase)

### MIPKG

Severity: **warning** · Auto-fix: **no**

'meta.package' is an internal API; use 'what' or package-qualified names instead

### SEPTUD

Severity: **warning** · Auto-fix: **no**

'serial' is deprecated; use 'serialport' instead

### SYDEUD

Severity: **warning** · Auto-fix: **no**

System.Data .NET interop is platform-specific and may not be available

### UIRSUD

Severity: **warning** · Auto-fix: **no**

'uiresume' used outside of a figure callback context

### UISUUD

Severity: **warning** · Auto-fix: **no**

Deprecated UI setup pattern; use modern App Designer patterns

## Examples

### Incorrect

```matlab
deploytool();           % MCADE: ADE function
s = serial('COM1');     % SEPTUD: use serialport instead
v = feature('version'); % FEATUD: undocumented internal function
import pkg.sub.*;       % IMPKG: unsupported import context
```

### Correct

```matlab
sp = serialport('COM1', 9600);
v = version;
```

## Configuration

```toml
[lint.rules.UNSUPPORTED_ENGINE]
severity = "warning"
skip_checks = ["IMPKG"]
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
