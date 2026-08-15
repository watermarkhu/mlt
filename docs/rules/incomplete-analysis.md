---
icon: lucide/wrench
---

# Incomplete Analysis

**Default severity:** Error
**Auto-fix:** No
**Category:** Incomplete Analysis
**Can be disabled:** Yes

## What this rule does

Implements the "Incomplete Analysis" checks from MATLAB's Code Analyzer.
These diagnostics report when the analysis itself was limited or could not
complete reliably: too many diagnostics, too many parse errors, files that
are too large, too deeply nested, or too long. They represent linter
internal limits and **cannot be disabled** by the user.

A single file-level engine (`IncompleteAnalysisEngine`) collects tree
metrics (node count, ERROR count, nesting depths) in one pass, then applies
each threshold check against the configured limits. Each diagnostic carries
the specific check ID (e.g. `TMMSG`, `MDEEP`).

Three related MATLAB checks are not emitted by this engine: `QUIT`
(analysis did not complete — the panic guard lives in the linter core),
`NOFIL` (file not found), and `RDERR` (unable to read file) are handled by
the CLI.

## Check IDs

### TMMSG

Severity: **error** · Auto-fix: **no**

More than 10,000 diagnostics generated

### TMSMS

Severity: **error** · Auto-fix: **no**

More than 1,000 parse errors generated

### MXASET

Severity: **error** · Auto-fix: **no**

File too complex to analyze

### NOSPC

Severity: **error** · Auto-fix: **no**

File too complex (nesting)

### MBIG

Severity: **error** · Auto-fix: **no**

File too large

### MDOTM

Severity: **error** · Auto-fix: **no**

Invalid file extension (not `.m`)

### MDMCR

Severity: **error** · Auto-fix: **no**

Deployed MATLAB file (`.ctf` or `.p`)

### EOFER

Severity: **error** · Auto-fix: **no**

Too many syntax errors

### EOFMI

Severity: **error** · Auto-fix: **no**

Incomplete file (ends in ERROR/MISSING)

### MDEEP

Severity: **error** · Auto-fix: **no**

Parentheses/brackets nested too deeply

### DEEPC

Severity: **error** · Auto-fix: **no**

Block comments nested too deeply

### DEEPN

Severity: **error** · Auto-fix: **no**

Functions nested too deeply

### DEEPS

Severity: **error** · Auto-fix: **no**

Statements nested too deeply

### TEXTL

Severity: **error** · Auto-fix: **no**

Text too long (line length)

## Examples

### Incorrect

```matlab
% EOFMI: file ends mid-block without a matching `end`
function f()
    if x > 0
        y = 1;
% MDEEP: parentheses nested deeper than the configured limit
z = ((((((((((((((((((((((((((((((((((((((((((1)))))))))))))))))))))))))))))))))))))))))))));
```

### Correct

```matlab
function f()
    if x > 0
        y = 1;
    end
end
z = 1;
```

## Configuration

```toml
[lint.rules.INCOMPLETE_ANALYSIS]
max_diagnostics = 10000
max_parse_errors = 1000
max_node_count = 100000
max_file_size = 1048576
max_paren_depth = 32
max_function_depth = 20
max_statement_depth = 15
max_line_length = 4096
```

See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
