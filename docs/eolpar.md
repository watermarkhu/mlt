# EOLPAR - Missing Closing Bracket at End of Line

**Default severity:** Error
**Auto-fix:** No
**Category:** Syntax Errors
**Can be disabled:** Yes

## What this rule does

Flags a missing closing bracket (`)`, `]`, or `}`) when the unclosed bracket is the last content on its line and more code follows later in the file. This is one of three checks (with `NOPAR2` and `ENDPAR`) that replace the former `NOPAR` check.

## Why this matters

- An unclosed bracket at the end of a line is easy to miss when scanning code
- The parse error can spill onto the following lines, making the real problem hard to spot
- Naming the exact location (end of line) tells the reader precisely where to add the missing bracket

## Examples

### Correct

```matlab
function foo()
    x = f(1);
    y = 2;
end
```

### Incorrect

```matlab
function foo()
    x = f(1;   % missing ) at end of line
    y = 2;
end
```

### Fixed

```matlab
function foo()
    x = f(1);
    y = 2;
end
```

## Configuration

This check is part of the `SYNTAX_ERRORS_ENGINE` file-level rule. Disable it in `.mlt.toml`:

```toml title=".mlt.toml"
[lint.rules.SYNTAX_ERRORS_ENGINE]
disabled_checks = ["EOLPAR"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable, e.g. `["EOLPAR", "ENDPAR"]` |

## Automatic fixes

None. The missing bracket must be inserted manually.

## Target node types

File-level check (`has_file_check`). It inspects `MISSING` nodes of kind `)`, `]`, or `}` whose line ends immediately after the node, with real content later in the file.

## Related rules

- `NOPAR2` — Missing closing bracket mid-file
- `ENDPAR` — Missing closing bracket at end of file
- `SYNER` — Generic syntax error
