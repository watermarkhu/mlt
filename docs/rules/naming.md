---
icon: lucide/type
---

# Naming Checks

**Default severity:** Info
**Auto-fix:** No
**Category:** Naming
**Can be disabled:** Yes

## What this engine does

The `NAMING_ENGINE` rule implements MATLAB's **Naming Checks** as a single generic engine that generates **81 static rule IDs** of the form `naming.<entity>.<checkType>`:

- **9 entity types**: `class`, `function`, `localFunction`, `method`, `nestedFunction`, `property`, `event`, `enumeration`, `variable`
- **9 check types**: `maxLength`, `minLength`, `regularExpression`, `requiredPrefix`, `disallowedPrefix`, `disallowedPhrase`, `requiredSuffix`, `disallowedSuffix`, `casing`

Entities are extracted via a full-tree DFS that classifies `function_definition`, `class_definition`, `property`, `enum`, and `assignment` nodes.

> **Note:** Only `maxLength` (default 63) and `minLength` (default 2) produce diagnostics without user configuration. The other seven check types require configuration.

## Configuration

Configure each check through its rule ID using the full-table form:

```toml
[lint.rules.naming.function.maxLength]
severity = "error"
max = 31

[lint.rules.naming.class.casing]
style = "PascalCase"

[lint.rules.naming.variable.requiredPrefix]
prefix = "v"
```

### Check parameters

| Parameter | Applies to | Description |
| --------- | ---------- | ----------- |
| `max` | `maxLength` | Maximum allowed name length |
| `min` | `minLength` | Minimum required name length |
| `pattern` | `regularExpression` | Regular expression the name must match |
| `prefix` | `requiredPrefix` / `disallowedPrefix` | Required or disallowed prefix string |
| `phrase` | `disallowedPhrase` | Disallowed substring |
| `suffix` | `requiredSuffix` / `disallowedSuffix` | Required or disallowed suffix string |
| `style` | `casing` | Casing style: `camelCase`, `PascalCase`, `snake_case`, `UPPER_CASE` |

## Related rules

- [Configuration](../configuration.md#per-engine-parameters) — naming engine parameters
