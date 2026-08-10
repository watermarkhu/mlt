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
| `TMMSG` | More than 10,000 diagnostics generated       \\| Post-lint (diagnostic count > threshold) |
| `TMSMS` | More than 1,000 parse errors generated       \\| Count ERROR nodes in tree |
| `MXASET` | File too complex to analyze                  \\| Node count > threshold |
| `QUIT` | Analysis did not complete                    \\| Engine panic guard (`catch_unwind` in `Linter::lint`) |
| `NOSPC` | File too complex (nesting)                   \\| Max nesting depth > threshold |
| `MBIG` | File too large                               \\| Source length > threshold |
| `NOFIL` | File not found                               \\| No-op (handled by CLI) |
| `MDOTM` | Invalid file extension                       \\| File extension != `.m` |
| `MDMCR` | Deployed MATLAB file                         \\| File extension == `.ctf` or `.p` |
| `RDERR` | Unable to read file                          \\| No-op (handled by CLI) |
| `EOFER` | Too many syntax errors                       \\| ERROR node count > threshold |
| `EOFMI` | Incomplete file                              \\| Last node is ERROR or MISSING |
| `MDEEP` | Parentheses/brackets nested too deeply       \\| Max `()`, `[]`, `{}` nesting depth |
| `DEEPC` | Block comments nested too deeply             \\| Nested `%{ %}` detection |
| `DEEPN` | Functions nested too deeply                  \\| Nested `function_definition` depth |
| `DEEPS` | Statements nested too deeply                 \\| Nested if/for/while/switch/try depth |
| `TEXTL` | Text too long                                \\| Max line length > threshold |

## Configuration

```toml
[lint.rules.INCOMPLETE_ANALYSIS]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
