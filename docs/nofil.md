# NOFIL - File Not Found

**Default severity:** Error
**Auto-fix:** No
**Category:** Incomplete Analysis
**Can be disabled:** No

## What this rule does

Emits an error when mlt is asked to lint a file that does not exist. This is a CLI-level guard, not a registered lint rule: mlt cannot analyze a file that cannot be opened, so it reports the missing path and continues with any remaining input files.

## Why this matters

- **Clear diagnostics**: A missing file should be reported as a missing file, not a generic read failure
- **Non-blocking**: One missing path should not stop mlt from linting the other files on the command line
- **Safety**: mlt never creates or overwrites a file at a path that does not exist, even with `--fix`

## Examples

### Incorrect

```bash
mlt script.m      # script.m does not exist
```

Output:

```text
script.m:1:1 [E] NOFIL: Unable to open file script.m. File is not found.

Found 1 issue in 1 file.
```

### Correct

```bash
mlt script.m      # script.m exists and is readable
```

No NOFIL diagnostic is produced.

## Configuration

This check is not configurable and cannot be disabled.

## Automatic fixes

None. mlt cannot fix a missing file.

## Target node types

None. This is a CLI-level guard that fires before parsing, based on the result of opening the input file.

## Related rules

- `RDERR` — Unable to read file (I/O errors other than not-found)
