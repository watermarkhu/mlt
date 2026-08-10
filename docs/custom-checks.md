---
icon: lucide/sliders-horizontal
---

# Custom Checks

**Default severity:** Warning
**Auto-fix:** No
**Category:** Custom Checks
**Can be disabled:** Yes

## What this engine does

The `CUSTOM_CHECKS` rule implements MATLAB's **Custom Checks** — configurable complexity and code-style metrics. All 25 checks are file-level and use thresholds that you set in the configuration.

## Check IDs

| Check ID | Description |
| -------- | ----------- |
| `SYSBANG` | System command used (`!`) |
| `FCNIL` | Function input count exceeds limit |
| `FCNOL` | Function output count exceeds limit |
| `FCNLL` | Function line count exceeds limit |
| `LLMNC` | Line length exceeds limit |
| `MNCSN` | Statement nesting depth exceeds limit |
| `DAFTC` | Too many children in tree |
| `DAFPV` | Too many persistent variables |
| `DAFCO` | Too many conditions in expression |
| `DAFBR` | Too many branches in switch/if |
| `DAFRT` | Too many return points |
| `DAFSC` | Too many semicolons on one line |
| `DAFNF` | Too many nested functions |
| `DAFCF` | Too many called functions |
| `DAFAF` | Too many anonymous functions |
| `DAFCV` | Too many local variables |
| `DAFCVC` | Too many local constants |
| `DAFVI` | Too many input arguments used |
| `DAFVO` | Too many output arguments used |
| `CYCCOM` | Cyclomatic complexity of function exceeds limit |
| `SCYCCOM` | Strict cyclomatic complexity exceeds limit |
| `ACYCCOM` | Average cyclomatic complexity exceeds limit |
| `MCYCCOM` | Method cyclomatic complexity exceeds limit |
| `MSCYCCOM` | Method strict cyclomatic complexity exceeds limit |
| `MACYCCOM` | Method average cyclomatic complexity exceeds limit |

## Configuration

```toml
[lint.rules.CUSTOM_CHECKS]
severity = "warning"
max_function_inputs = 5
max_function_outputs = 3
max_function_lines = 200
max_line_length = 100
max_nesting_depth = 4
max_cyclomatic_complexity = 10
max_strict_cyclomatic_complexity = 5
max_avg_cyclomatic_complexity = 6
max_branches = 10
max_return_points = 3
max_nested_functions = 3
max_anonymous_functions = 3
max_local_variables = 20
max_local_constants = 10
max_called_functions = 50
max_semicolons_per_line = 5
max_tree_children = 100
max_persistent_variables = 3
max_conditions = 10
max_input_args_used = 20
max_output_args_used = 20
```

### Parameters

| Parameter | Default | Description |
| --------- | ------- | ----------- |
| `max_function_inputs` | `0` (off) | Maximum function inputs |
| `max_function_outputs` | `0` (off) | Maximum function outputs |
| `max_function_lines` | `0` (off) | Maximum lines per function |
| `max_line_length` | `0` (off) | Maximum line length |
| `max_nesting_depth` | `0` (off) | Maximum statement nesting depth |
| `max_cyclomatic_complexity` | `0` (off) | Maximum cyclomatic complexity |
| `max_strict_cyclomatic_complexity` | `0` (off) | Maximum strict cyclomatic complexity |
| `max_avg_cyclomatic_complexity` | `0` (off) | Maximum average cyclomatic complexity |
| `max_branches` | `0` (off) | Maximum branches in if/switch |
| `max_return_points` | `0` (off) | Maximum return points per function |
| `max_nested_functions` | `0` (off) | Maximum nested functions |
| `max_anonymous_functions` | `0` (off) | Maximum anonymous functions |
| `max_local_variables` | `0` (off) | Maximum local variables |
| `max_local_constants` | `0` (off) | Maximum local constants |
| `max_called_functions` | `0` (off) | Maximum called functions |
| `max_semicolons_per_line` | `0` (off) | Maximum semicolons on one line |
| `max_tree_children` | `0` (off) | Maximum children in a node |
| `max_persistent_variables` | `0` (off) | Maximum persistent variables |
| `max_conditions` | `0` (off) | Maximum conditions in an expression |
| `max_input_args_used` | `0` (off) | Maximum input arguments used |
| `max_output_args_used` | `0` (off) | Maximum output arguments used |

A threshold of `0` disables that check.

## Related rules

- [Configuration](configuration.md#per-engine-parameters) — custom-checks parameters
