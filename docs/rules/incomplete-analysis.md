---
icon: lucide/wrench
---

# Incomplete Analysis

**Default severity:** Error
**Auto-fix:** No
**Category:** Incomplete Analysis
**Can be disabled:** Yes

## What this engine does

The `INCOMPLETE_ANALYSIS` rule implements the MATLAB Code Analyzer checks in the **Incomplete Analysis** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `TMMSG`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `TMMSG` | More than 10,000 diagnostics generated |
| `TMSMS` | More than 1,000 parse errors generated |
| `MXASET` | File too complex to analyze |
| `QUIT` | Analysis did not complete |
| `NOSPC` | File too complex (nesting) |
| `MBIG` | File too large |
| `NOFIL` | File not found |
| `MDOTM` | Invalid file extension |
| `MDMCR` | Deployed MATLAB file |
| `RDERR` | Unable to read file |
| `EOFER` | Too many syntax errors |
| `EOFMI` | Incomplete file |
| `MDEEP` | Parentheses/brackets nested too deeply |
| `DEEPC` | Block comments nested too deeply |
| `DEEPN` | Functions nested too deeply |
| `DEEPS` | Statements nested too deeply |
| `TEXTL` | Text too long |

## Configuration

```toml
[lint.rules.INCOMPLETE_ANALYSIS]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
