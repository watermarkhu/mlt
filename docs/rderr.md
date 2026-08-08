# RDERR - Unable to Read File

**Default severity:** Error
**Auto-fix:** No
**Category:** Incomplete Analysis
**Can be disabled:** No

## What this rule does

Emits an error when mlt cannot read an input path for any reason other than the file not existing — for example, the path is a directory, or permissions prevent reading. This is a CLI-level guard, not a registered lint rule: mlt cannot analyze a file it cannot read, so it reports the failure and continues with any remaining input files.

## Why this matters

- **Clear diagnostics**: A read failure is reported explicitly instead of aborting with a generic error
- **Non-blocking**: One unreadable path should not stop mlt from linting the other files on the command line
- **Safety**: mlt never writes to an unreadable or non-file path, even with `--fix`

## Examples

### Incorrect

```bash
mlt somedir/       # path is a directory
mlt secret.m       # permissions prevent reading
```

Output:

```text
somedir/:1:1 [E] RDERR: Unable to read file somedir/.

Found 1 issue in 1 file.
```

### Correct

```bash
mlt script.m      # script.m exists and is readable
```

No RDERR diagnostic is produced.

## Configuration

This check is not configurable and cannot be disabled.

## Automatic fixes

None. mlt cannot fix an unreadable file.

## Target node types

None. This is a CLI-level guard that fires before parsing, based on the result of opening the input file.

## Related rules

- `NOFIL` — File not found
