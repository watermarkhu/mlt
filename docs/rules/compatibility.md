---
icon: lucide/archive
---

# Compatibility Considerations

**Default severity:** Warning
**Auto-fix:** No
**Category:** Compatibility
**Can be disabled:** Yes

## What this engine does

The `COMPAT` rule implements the **Compatibility Considerations** checks from MATLAB's Code Analyzer. It is a **data-driven** engine: check metadata (ID, message, severity) lives in `data/compatibility.toml` and is loaded into a lookup table at compile time.

On each `function_call` or `command` node the engine extracts the function name and performs an O(1) lookup. A matched name may produce **multiple** diagnostics (e.g. `tcpip` → TCPC + TCPS); every matching check ID is emitted.

## Check IDs

This engine covers **1,924 data entries** grouped by the `category` field in `data/compatibility.toml`:

| Group | Entries |
| ----- | ------- |
| Compatibility Considerations | 1,012 |
| Forward Compatibility | 7 |
| Behavior Changes | 905 |

These map onto the MATLAB Code Analyzer inventory targets (Compatibility Considerations 890, Behavior Changes Low Reliability 265, Upcoming Behavior Changes Low Reliability 632, Forward Compatibility 7); the data file carries a few extra entries that share IDs across the low-reliability tables.

In addition, **65 of 68** generic AST-pattern checks (`function_name = ""`) that the lookup table cannot match are implemented in `compatibility/generic/` — these cover property/attribute removals, removed options and input arguments, forward-compatibility version gates, global/import scope, and behavior-change figure/axes properties.

The complete ID table is generated into [../rules.md](../rules.md).

## Configuration

The lookup engine itself takes no parameters. Per-check severity can be overridden by rule ID or by category:

```toml
[lint.categories]
compatibility = "warn"

[lint.rules]
COMPAT = "error"
```

## Automatic fixes

None — these checks report deprecated or removed usage; the replacement is described in the diagnostic message.

## Target node types

- `function_call`
- `command`
- (file-level pass for the generic AST checks)

## Related rules

- [Suggested Improvements](suggested-improvements.md) — replacement-function suggestions
- [Configuration](../configuration.md#per-engine-parameters)
