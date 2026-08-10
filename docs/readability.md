---
icon: lucide/eye
---

# Readability Improvements

**Default severity:** Info
**Auto-fix:** No
**Category:** Readability
**Can be disabled:** Yes

## What this engine does

The `READABILITY_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Readability Improvements** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `ASGSL`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `ASGSL` | Assignment inside a conditional expression |
| `COMNL` | Newline following comma acts as a row separator in a matrix |
| `SPERR` | Prefer a message identifier for error |
| `SPWRN` | Prefer a message identifier for warning |
| `NCHKE` | Use narginchk/nargoutchk for argument validation |
| `DSPSP` | Prefer fprintf/disp over sprintf+disp for display |
| `DSPSY` | Prefer fprintf/disp over system-based display |
| `STLOW` | Unnecessary UPPER/LOWER call in a comparison |
| `FLUDLR` | Nested flipud(fliplr(x))/fliplr(flipud(x)) should use rot90(x, 2) |
| `RPMT1` | Trivial multiplication by 1 |
| `RPMT0` | Trivial addition/subtraction of 0 |
| `RPMTT` | Boolean tautology (true \\|\\| ...) |
| `RPMTF` | Boolean contradiction (false && ...) |
| `RPMTI` | Trivial multiplication by an identity-like expression |
| `RPMTN` | Trivial negation patterns |
| `PSIZE` | Use numel instead of prod(size(x)) |
| `LOGSUM` | Use nnz instead of sum for logical vectors |
| `LOGL` | Prefer any/all over manual logical reduction |
| `ISCHR` | Use ischar(x) instead of isa(x,'char') |
| `ISSTR` | Use isstring(x) instead of isa(x,'string') |
| `ISLOG` | Use islogical(x) instead of isa(x,'logical') |
| `ISCEL` | Use iscell(x) instead of isa(x,'cell') |
| `IJCL` | i or j used as a variable (shadows the complex unit) |
| `ISMAT` | Use ismatrix(x) instead of ndims(x)==2 |
| `ISROW` | Use isrow(x) instead of size(x,1)==1 |
| `ISCOL` | Use iscolumn(x) instead of size(x,2)==1 |
| `NBRAK2` | Unnecessary brackets in indexing |
| `MFAMB` | Cannot determine whether a name is a variable or function |
| `FVINR` | Add an (Input) attribute to arguments blocks for readability |
| `STREMP` | Use strlength(s)==0 instead of strcmp(s,'') |
| `STRCL1` | Use strlength/strtrim instead of string-cleaning wrappers |
| `STRCLFH` | Use strip instead of string-cleaning wrappers |
| `STRIFCND` | Simplify if-conditions involving string comparisons |
| `CHARTEN` | Use newline instead of char(10) |
| `SPRINTFN` | Use num2str over simple sprintf for number formatting |

## Configuration

```toml
[lint.rules.READABILITY_ENGINE]
disabled_checks = ["XXXX"]   # Turn off specific checks
```

See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.
