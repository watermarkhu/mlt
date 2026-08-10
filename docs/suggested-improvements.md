---
icon: lucide/lightbulb
---

# Suggested Improvements

**Default severity:** Info
**Auto-fix:** No
**Category:** Suggested Improvements
**Can be disabled:** Yes

## What this engine does

The `SUGGESTED_IMPROVEMENTS` rule implements the **Suggested Improvements** checks from MATLAB's Code Analyzer. It is a **data-driven** engine: check metadata lives in `data/suggested_improvements.toml` and is loaded into a lookup table at compile time.

Each check pairs a function name with a recommended replacement (e.g. `csvread` → `readmatrix`). The engine matches `function_call` and `command` nodes by name and emits the specific check ID with the replacement suggestion.

## Check IDs

This engine covers **243 check IDs**. Examples:

| Check ID | Message (summary) |
| -------- | ----------------- |
| `CSVRD` | Use `readmatrix` instead of `csvread` |
| `DLMRD` | Use `readmatrix` instead of `dlmread` |
| `XLSRD` | Use `readmatrix`/`readtable` instead of `xlsread` |
| `CSVWT` | Use `writematrix` instead of `csvwrite` |
| `DLMWT` | Use `writematrix` instead of `dlmwrite` |
| `XLSWT` | Use `writematrix`/`writetable` instead of `xlswrite` |
| `ISDIR` | Use `isfolder` instead of `isdir` |
| `HIST` | Use `histogram` instead of `hist` |
| `HISTC` | Use `histcounts` instead of `histc` |

The complete ID table is generated into [rules.md](rules.md).

## Configuration

```toml
[lint.categories]
suggested-improvements = "info"

[lint.rules]
SUGGESTED_IMPROVEMENTS = "off"
```

## Automatic fixes

None — the diagnostic message names the recommended replacement.

## Target node types

- `function_call`
- `command`

## Related rules

- [Compatibility](compatibility.md) — removed/deprecated functions
- [Configuration](configuration.md#per-engine-parameters)
