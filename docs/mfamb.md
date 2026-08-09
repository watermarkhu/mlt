# MFAMB - Ambiguous Identifier Assumed to be a Function

**Default severity:** Info
**Auto-fix:** No
**Category:** Readability
**Can be disabled:** Yes

## What this rule does

Flags an identifier used with function-call syntax (`name(...)`) that has also been assigned as a variable in the same scope (or an enclosing scope). MATLAB's Code Analyzer cannot determine whether such an identifier is a variable (array/cell indexing) or a function, and assumes it is a function.

The official message is:

> Code Analyzer cannot determine whether VAR_NAME is a variable or a function, and assumes it is a function.

This rule is a sub-check of the `READABILITY_ENGINE` and is implemented as a file-level analysis using the symbol table.

## Why this matters

- **Ambiguity**: When a name is both a variable and a function callee, the intent is unclear to both the reader and the analyzer
- **Runtime behavior**: MATLAB resolves the ambiguity at runtime; a name that is a variable is indexed, while an unresolved name is treated as a function call — the outcome may not match the author's intent
- **Readability**: Distinct naming for variables versus functions (e.g., `data` vs. `computeData`) makes the code self-documenting

## Examples

### Incorrect

```matlab
somevar = 5;
y = somevar(1);     % somevar is a variable; is this indexing or a function call?
```

### Correct

```matlab
data = 5;
y = data(1);        % consistent variable usage

result = myfunc(1); % distinct function name
```

## Pragmatic interpretation

Because `tree-sitter-matlab` cannot distinguish array/cell indexing (`A(i)`) from function calls (`f(x)`), mlt uses a conservative interpretation of this check:

- The callee must be a bare identifier (not a field expression such as `obj.method(...)`)
- The name must be defined as a variable (assignment, input argument, for-loop iterator, global/persistent, lambda parameter, or output argument) in the current or an enclosing scope
- Well-known built-in functions that are commonly used with indexing-like syntax (`length`, `size`, `numel`, `abs`, `sum`, `max`, `min`, `mean`) are always treated as functions and are not flagged
- Indexed assignment left-hand sides (`x(1) = 5`) are never flagged, since the left-hand side must be a variable

### Limitation

A variable that is only assigned later in the file is still considered "in scope" for the entire file-level script/function scope. The rule does not do order-sensitive dataflow analysis, so it may report a diagnostic even when the assignment appears after the call. It also cannot know about variables created dynamically (e.g., by `eval` or `assignin`).

## Configuration

```toml title=".mlt.toml"
[lint.rules.READABILITY_ENGINE]
severity = "info"
disabled_checks = ["MFAMB"]
```

### Parameters

| Parameter | Type | Default | Description |
| --------- | ---- | ------- | ----------- |
| `severity` | string | `"info"` | Severity level (`"error"`, `"warn"`, `"info"`, `"off"`) |
| `disabled_checks` | array of strings | `[]` | Check IDs to disable within the readability engine |

To disable MFAMB while keeping other readability checks, add `"MFAMB"` to `disabled_checks`. The rule cannot be configured independently, since it is a sub-check of `READABILITY_ENGINE`.

## Automatic fixes

This rule provides no automatic fixes — renaming the variable or the function call requires semantic knowledge of the code.

## Target node types

This is a file-level check. It inspects `function_call` nodes during a full-file traversal and consults the symbol table to determine whether the callee name is also a defined variable.

## Related rules

- `README` - Readability improvement engine
