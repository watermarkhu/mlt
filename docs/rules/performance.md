---
icon: lucide/zap
---

# Performance Improvements

**Default severity:** Info
**Auto-fix:** No
**Category:** Performance
**Can be disabled:** Yes

## What this engine does

The `PERFORMANCE_ENGINE` rule implements the MATLAB Code Analyzer checks in the **Performance Improvements** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `AGROW`).

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `AGROW` | Variable appears to change size on every loop iteration; consider preallocating |
| `SAGROW` | Sliced variable appears to grow inside a loop |
| `PFBNS` | Prefer broadcasting syntax over bsxfun calls |
| `RGXP1` | Use regexp with output arguments instead of regexpi when case is known |
| `RGXPI` | Use regexpi output argument form |
| `TRIM1` | Use strtrim instead of deblank for trimming leading/trailing whitespace |
| `TRIM2` | Prefer strtrim over deblank |
| `STTOK` | Prefer split/strsplit over strtok |
| `STNCI` | Use startsWith instead of comparing the first characters of a string |
| `STCCS` | Use strcmp for character-vector comparison |
| `FNDSB` | Prefer find(x > 0, 1) over find(x, 1) style patterns |
| `SFLD` | Use dynamic field names instead of setfield |
| `GFLD` | Use dynamic field names instead of getfield |
| `CCAT` | Concatenate cell arrays using [] instead of extracting and reconstructing |
| `CCAT1` | {A{I}} can usually be replaced by A(I) or A(I)' |
| `ISMT` | Use ismatrix instead of comparing ndims to 2 |
| `ISCL` | Use isscalar instead of numel(x)==1 |
| `ST2NM` | Prefer str2double over str2num |
| `FLPST` | Prefer flip/rot90 over flipud/fliplr where equivalent |
| `MXFND` | Use max with a single output when only the value is needed |
| `EFIND` | Use the faster find form for simple conditions |
| `EXIST` | Use isfile/isfolder instead of exist |
| `UDIM` | Use numel instead of size for a single dimension when the array is 1-D |
| `FREAD` | Use fread with fewer output arguments when possible |
| `N2UNI` | Use unique instead of manual sort+diff patterns |
| `TNMLP` | Prefer strlength over numel for strings |
| `MINV` | Use A\b instead of inv(A)*b |
| `LAXES` | Prefer axes() with explicit arguments |
| `MMTC` | Use mtimes/mtimesc scalar-matrix shortcuts |
| `MRPBW` | Prefer repmat-avoiding broadcasting |
| `SPRIX` | Use sparse indexing forms |
| `TRSRT` | Use issorted instead of manual sort comparisons |
| `GRIDD` | Prefer ndgrid over meshgrid where appropriate |
| `AND2` | Use && instead of & for scalar logical AND |
| `OR2` | Use \|\| instead of \| for scalar logical OR |
| `CLALL` | Avoid clear all; it usually decreases performance |
| `CLCLS` | Avoid clear classes |
| `CLFUNC` | Avoid clear functions |
| `CLJAVA` | Avoid clear java |
| `CLMEX` | Avoid clear mex |
| `CLEAR0ARGS` | clear with no arguments is often unnecessary |

## Configuration

```toml
[lint.rules.PERFORMANCE_ENGINE]
skip_checks = ["AGROW"]   # Turn off specific checks
```

See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.
