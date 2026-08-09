# FVINR - Input Attribute

**Default severity:** Info
**Auto-fix:** Yes
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

Flags `arguments` validation blocks that do not declare an attribute such as `(Input)`. MATLAB's Code Analyzer recommends explicitly labeling input arguments blocks with the `(Input)` attribute for readability, mirroring how output arguments blocks use `(Output)`.

FVINR is a sub-check of the `READABILITY_ENGINE` rule. It is enabled by default and can be disabled individually via the `disabled_checks` configuration parameter.

## Why this matters

- **Clarity**: An `arguments (Input)` block makes the direction of data flow explicit at a glance — the block validates values coming *into* the function
- **Consistency**: Functions that declare `(Output)` on output blocks but omit `(Input)` on input blocks are inconsistent and harder to read
- **Self-documentation**: Attributes convey intent (validation vs. mutation) without needing to read the function signature

## Examples

### Correct

```matlab
function y = f(x)
    arguments (Input)
        x (1,1) double
    end
    arguments (Output)
        y
    end
    y = x * 2;
end
```

### Incorrect

```matlab
function y = f(x)
    arguments
        x (1,1) double
    end
    y = x * 2;
end
```

### Fixed

```matlab
function y = f(x)
    arguments (Input)
        x (1,1) double
    end
    y = x * 2;
end
```

## Configuration

FVINR is a sub-check of `READABILITY_ENGINE`, so it is configured through that rule's `disabled_checks` parameter:

```toml title=".mlt.toml"
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["IJCL", "FVINR"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Check IDs within `READABILITY_ENGINE` to disable (e.g., `"FVINR"`) |

### `disabled_checks`

A list of readability sub-check IDs to turn off while keeping the rest of the engine active. To disable only FVINR:

```toml
disabled_checks = ["FVINR"]
```

## Automatic fixes

This rule inserts ` (Input)` immediately after the `arguments` keyword:

```diff
  function y = f(x)
-     arguments
+     arguments (Input)
          x (1,1) double
      end
      y = x * 2;
  end
```

The fix is safe: it only adds a declaration attribute and does not change program behavior.

## Target node types

This rule triggers on the following tree-sitter node type:

- `arguments_statement` — an `arguments ... end` block that has no `attributes` child

Blocks that already carry an attribute (`(Input)` or `(Output)`) are not flagged.

## Related rules

- `READABILITY_ENGINE` — the parent engine that dispatches FVINR and other readability sub-checks
