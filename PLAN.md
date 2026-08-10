# PLAN.md — MATLAB Code Analyzer Feature Parity for mlt

This document tracks the implementation plan for bringing mlt to full feature
parity with MATLAB's Code Analyzer (~2,680 checks).

**Decisions made:**
- Rule IDs match MATLAB's native check IDs exactly (e.g., `NOSEMI`, `AGROW`, `naming.class.casing`).
- Compatibility/behavior-change checks (~1,800) use a data-driven TOML lookup table.
- Naming checks (81) use a single generic engine with 81 separate rule IDs.
- Custom checks (25) use a single engine with per-check thresholds.
- Rules auto-register via the `inventory` crate — no manual `lib.rs` arrays.
- Category-level config is supported (`[lint.categories]` in `.mlt.toml`).
- Changes land via **stacked pull requests** (see below).
- Rule context (description, severity, examples) for every check MUST be looked up on the
  official MATLAB Code Analyzer check index:
  `https://www.mathworks.com/help/matlab/matlab_env/index-of-code-analyzer-checks.html`.
  MathWorks blocks plain HTTP/webfetch with HTTP 403, so the page is read with the
  **Playwright MCP browser** (`playwright_browser_navigate` → `playwright_browser_find` →
  `playwright_browser_snapshot`). A local archive of the page also exists at
  `~/.local/share/opencode/tool-output/tool_fe291ac62001Qy9pI2rEjqHVAK` as fallback.
- Each new rule is implemented via the **rule-pipeline** (see
  `.opencode/skill/rule-pipeline/`): planning happens in the main session (it owns the
  single Playwright MCP instance), then each rule is reviewed and implemented via
  independent parallel **Reviewer → Implementer** subagent flows (`.opencode/agent/`).
  Subagents never use Playwright; rule context is passed to them as plan text.

---

## Development Workflow: Stacked Pull Requests

This repository uses **stacked pull requests** to land changes as a chain of
small, independently reviewable PRs instead of one large PR. Each PR targets
the branch of the PR below it, forming an ordered stack that lands on `feat`.

See the [GitHub stacked PRs quickstart](https://docs.github.com/en/pull-requests/get-started/stacked-prs-quickstart)
for the official guide.

### Setup (once per machine)

```bash
gh extension install github/gh-stack
gh auth login
```

Requires `gh` ≥ 2.90 and Git ≥ 2.20.

### Daily flow

1. **Start a stack** from the trunk (`feat`):

   ```bash
   gh stack init --base feat        # prompts for the first branch name
   ```

   To turn existing branches into a stack, list them in dependency order:

   ```bash
   gh stack init --base feat feature/foo feature/bar feature/baz
   ```

2. **Work and commit** on the current branch:

   ```bash
   # ... write code ...
   git add .
   git commit -m "helpful message"
   ```

3. **Add the next logical unit** on top of the stack:

   ```bash
   gh stack add BRANCH-NAME
   # ... write code ...
   git add .
   git commit -m "next unit"
   ```

   Or stage, commit, and branch in one step:

   ```bash
   gh stack add -Am "next unit"
   ```

4. **Push and submit** the PRs (each PR is auto-linked to its base branch):

   ```bash
   gh stack push
   gh stack submit
   ```

5. **Inspect the stack** at any time:

   ```bash
   gh stack view
   ```

### Conventions

- **Trunk:** `feat` is this repo's integration branch; the bottom PR of every
  stack targets it.
- **One logical unit per branch.** Split work so each branch is independently
  reviewable — e.g., one branch for clippy cleanups, the next for a rule
  engine plus its tests, and docs/plan updates on top.
- **Keep stacks shallow** (2–4 branches). Deep stacks are hard to review and
  prone to merge conflicts.
- **Review and merge bottom-up.** Each PR's diff shrinks as its dependencies
  land, keeping downstream PRs small.
- **Sync with trunk** before merging: rebase the bottom branch onto `feat`,
  then each branch onto its parent.

---

## Current State

### Phase 3 Waves A–D ✅ (merged into `feat` via PRs #8/#9/#10/#12)

~185 checks added across four stacked-PR waves using the rule-pipeline
(`.opencode/skill/rule-pipeline/`). Phase 3 categories now sit at **~730 of 771
(94.7%)**; the full MATLAB inventory is **~730 of 2,680 (~27%)**. See the
**Phase 3: Rule Implementation** section below for the summary and remaining-work
list.

| Wave | Branch (merged) | Checks | Modules |
|------|-----------------|--------|---------|
| A | `feature/phase3-readability` (#8) | 8 | `readability.rs`, `incomplete_analysis.rs`, `mlt_core/linter.rs`, `mlt_cli/main.rs` |
| B | `feature/phase3-syntax-errors` (#9) | 30 | `syntax_errors.rs` |
| C | `feature/phase3-language-spec` (#10) | 108 | `language_spec.rs`, `bugs.rs` (ID renames) |
| D | `feature/phase3-good-practices` (#12) | 39 (+24 deferred) | `good_practices.rs` |

**Remaining Phase 3 work:** ~39 checks (Good Practices 26, Language Spec 11,
System Objects 2) plus 24 deferred Good Practices checks requiring type/flow analysis.

### Coverage — two denominators

- **Full MATLAB Code Analyzer inventory: ~730 of 2,680 checks (~27%).** The bulk of
  the remaining 73% is **data-driven** categories (Compatibility ~1,803, Behavior
  Changes ~905) that are Phase 4 data-file expansion, not Phase 3 rule code.
- **Phase 3 in-scope categories only: ~730 of 771 (~94%).** The remaining ~41 checks
  (Good Practices 26 + 11 Language Spec + 2 System Objects) plus 24 deferred Good
  Practices checks.

### Implemented Engines (pre-wave baseline, now superseded by the table above)

| Module | File | Functional Check IDs | Approach |
|--------|------|---------------------|----------|
| NOSEMI | `nosemi.rs` | 1 | Node-level, auto-fix |
| Compatibility | `compatibility.rs` + `data/compatibility.toml` | ~1,794 (data-driven; 68 generic entries unmatchable) | Data-driven HashMap lookup → `Vec` for multi-function names |
| Naming | `naming.rs` | 81 (63 are no-ops without user config) | File-level traversal, configurable |
| Custom Checks | `custom_checks.rs` | 25 | File-level metrics, configurable thresholds |

**Total functional check IDs: ~2,500 of 2,680 (~93%)** after the Phase 3 waves
and Phase 4 data expansion. (Phase 4 populated all 1,794 Compatibility/Behavior/
Forward check IDs in the data file; the remaining ~180 gap is mostly the 68
generic AST-pattern entries plus other-category leftovers.)

### Core Infrastructure

| Feature | Status |
|---------|--------|
| Rule trait with `category()`, `can_be_disabled()`, `has_file_check()` | Done |
| Category enum (22 variants) | Done |
| Category-level config (`[lint.categories]`) | Done |
| `inventory` auto-registration | Done |
| File-level dispatch optimization | Done |
| Overlapping fix detection | Done |
| Config error reporting (non-silent) | Done |
| Symbol table / scope analysis | Done (Phase 2.1) |
| Control flow analysis | Done (Phase 2.2) |
| Cross-file analysis | Not started |
| Inline suppression (`%#ok<RULE>`) | Not started |

---

## Target: MATLAB Code Analyzer Check Inventory

| # | Category | Count | Severity | Notes |
|---|----------|-------|----------|-------|
| 1 | Incomplete Analysis | 17 | Error | **17/17 done** — Wave A completed QUIT, NOFIL, RDERR; all `can_be_disabled = false`; linter-internal limits |
| 2 | Syntax Errors | 50 | Error | **50/50 done** — Wave B completed 30 checks (BADCT, BADFP, BADHBH, BADHBB, BADHBHT, BADHBBT, HEXTOOLONG, BINARYTOOLONG, DOUQT, STRIN, INBLK, RESWD, UNSET, LHROW, NOPAR2, EOLPAR, ENDCT2, ENDCT3, ENDCT4, MCPLD, SBTMP, BADNOT, BADNOTLHS, ENDPAR, VTPOD, SYNEND, FVACI, FVACS, FVAMI, FVSYN) |
| 3 | Language Specification Errors | 157 | Error | **146/157 done** — Wave C added 108 checks; 11 unresolved vs index count |
| 4 | Bugs | 35 | Error | Suspicious patterns, logic errors |
| 5 | Custom Checks | 25 | Warning | **Done** — complexity/style metrics |
| 6 | Naming Checks | 81 | Info | **Done** — 9 entities × 9 check types |
| 7 | Compatibility Considerations | 891 | Error/Warning | **890/891 done** — Phase 4 populated the data file |
| 8 | Forward Compatibility | 7 | Error/Warning | **7/7 done** — Phase 4 (FCLEN, FCCPV, FCDQS, FCFAV, FCHBL, FCLFS, FCNVA) |
| 9 | Good Practices | 106 | Warning | **80/106 done** — Wave D added 39; 24 deferred (type/flow analysis), 2 pending |
| 10 | Unset Variables | 6 | Warning | **6/6 done** |
| 11 | Unused Constructions | 17 | Warning/Info | **17/17 done** |
| 12 | Suggested Improvements | 243 | Info | Data-driven (function replacement suggestions) |
| 13 | Readability Improvements | 36 | Info | **36/36 done** — Wave A completed COMNL, STLOW, FLUDLR, MFAMB, FVINR |
| 14 | Formatting Suggestions | 8 | Info | **8/8 done** — NOSEMI + 7 formatting checks |
| 15 | Performance Improvements | 41 | Info | Pattern matching on AST |
| 16 | MATLAB for Code Generation | 20 | Error | **20/20 done** |
| 17 | Fixed-Point Messages | 1 | Warning | Specialized |
| 18 | MATLAB Compiler (Deployment) | 10 | Error/Warning | **10/10 done** |
| 19 | System Objects | 9 | Error/Warning | **7/9 done** (SOTUNPROP variants consolidated) |
| 20 | Unsupported Features | 13 | Warning | **13/13 done** |
| 21 | Behavior Changes | 5 | Warning | In data file (5 entries) |
| 22 | Behavior Changes (Low Reliability) | 265 | Warning | **265/265 done** — Phase 4 (JAPIEXT* + non-JAPIEXT) |
| 23 | Upcoming Behavior Changes | 3 | Warning | In data file as generic |
| 24 | Upcoming Behavior Changes (Low Reliability) | 632 | Warning | **632/632 done** — Phase 4 (ROSDFMISSING + 631 JAPIEXT*) |
| 25 | Code Analyzer Configuration Issues | 4 | Error | **4/4 done** |
| | **TOTAL** | **2,680** | | |

---

## Phase 0: Architecture Fixes ✅ COMPLETE

All items completed and verified.

### 0.1 — Fix Config Parsing Bug ✅
- **File:** `crates/mlt_core/src/config.rs`
- **Problem:** Early `return` in `from_config_file()` when a rule has `severity = "off"` in full-table form dropped all subsequently-processed rules.
- **Fix:** Replaced `return Ok(...)` with `continue` in the loop.
- **Test added:** `test_full_table_off_does_not_drop_subsequent_rules`

### 0.2 — Fix `rule_params()` Silent Error Swallowing ✅
- **File:** `crates/mlt_core/src/config.rs`
- **Problem:** `value.try_into::<T>().ok()` silently returned `Default` on malformed config.
- **Fix:** Added `eprintln!` warning when deserialization fails, so users see config errors.

### 0.3 — Add Rule Categories ✅
- **File:** `crates/mlt_core/src/rule.rs`
- Added `Category` enum with 22 variants matching MATLAB Code Analyzer groups.
- Added `fn category(&self) -> Category` to `Rule` trait.
- **File:** `crates/mlt_core/src/config.rs`
- Added `[lint.categories]` section support with `HashMap<Category, Option<Severity>>`.
- Added `is_rule_enabled_for_category()` and `effective_severity()` methods.
- Resolution order: per-rule > per-category > rule default.

### 0.4 — Auto-Registration with `inventory` ✅
- **File:** `crates/mlt_rules/src/lib.rs`
- Replaced static `RULE_FACTORIES` const slice with `inventory::collect!(RuleRegistration)`.
- Each rule module self-registers via `inventory::submit!`.
- Adding a rule requires only: create module file + add `pub mod` line. No other edits.

### 0.5 — File-Level Dispatch Optimization ✅
- **File:** `crates/mlt_core/src/registry.rs`
- Added `file_check_indices: Vec<usize>` populated at construction from `rule.has_file_check()`.
- **File:** `crates/mlt_core/src/linter.rs`
- Changed file-level dispatch to iterate only `file_check_rules()` instead of all rules.

### 0.6 — Overlapping Fix Detection ✅
- **File:** `crates/mlt_cli/src/main.rs`
- Added overlap detection in `apply_fixes()`: if a fix's byte range overlaps with the previously applied fix, skip it and emit a warning.

### 0.7 — Rename M001 → NOSEMI ✅
- Renamed `m001_trailing_semicolon.rs` → `nosemi.rs`.
- Updated rule ID from `"M001"` to `"NOSEMI"`.
- Updated severity from `Warning` to `Info` (matching MATLAB's Formatting Suggestions).
- Added `fn category(&self) -> Category { Category::Formatting }`.
- Updated docs, `zensical.toml`, and `rules.md`.

### 0.8 — Rule Metadata ✅
- Added `fn can_be_disabled(&self) -> bool { true }` to `Rule` trait (default true).
- Added `fn has_file_check(&self) -> bool { false }` to `Rule` trait (default false).

---

## Phase 1: Data-Driven Rule Engines ✅ COMPLETE

### 1.1 — Compatibility Lookup Engine ✅
- **File:** `crates/mlt_rules/src/compatibility.rs` (245 lines)
- **Data:** `crates/mlt_rules/src/data/compatibility.toml` (323 entries, 2,350 lines)
- Single `CompatibilityEngine` targeting `function_call` and `command` nodes.
- `LazyLock<HashMap>` built from TOML at first access; O(1) lookup per function name.
- Diagnostics carry the specific MATLAB check ID (e.g., `DPSD`), not the meta-ID.
- **275 entries** have real function names and are actively matchable.
- **48 entries** are generic (empty `function_name`); require future AST pattern matching.
- **Known issue:** Duplicate function names (`tcpip`, `legend`, `web`) cause one check ID to be shadowed.
- **Remaining work:** Populate remaining ~1,480 entries to reach full 1,803 coverage.

### 1.2 — Generic Naming Engine ✅
- **File:** `crates/mlt_rules/src/naming.rs` (849 lines)
- Single `NamingEngine` registered as file-level rule.
- Generates 81 static rule IDs (`naming.<entity>.<checkType>`).
- 9 entity types: class, function, localFunction, method, nestedFunction, property, event, enumeration, variable.
- 9 check types: maxLength, minLength, regularExpression, requiredPrefix, disallowedPrefix, disallowedPhrase, requiredSuffix, disallowedSuffix, casing.
- Entity extraction via full-tree DFS classifying `function_definition`, `class_definition`, `property`, `enum`, `assignment` nodes.
- **Note:** 7 of 9 check types require user config to produce diagnostics. Only `maxLength` (default 63) and `minLength` (default 2) fire without config.

### 1.3 — Custom Checks Framework ✅
- **File:** `crates/mlt_rules/src/custom_checks.rs` (1,286 lines)
- Single `CustomChecksEngine` as file-level rule.
- All 25 check IDs implemented with configurable thresholds.
- Cyclomatic complexity (standard and strict), nesting depth, line-based metrics, function-level metrics.
- **Known issue:** DAFSC semicolons-per-line counts semicolons inside strings/comments.
- **Known issue:** Average complexity uses integer division (truncates fractions).

---

## Phase 2: Semantic Analysis Infrastructure ✅ COMPLETE

### 2.1 — Symbol Table ✅
- **File:** `crates/mlt_rules/src/analysis/symbols.rs` (1,355 lines)
- `SymbolTable::build(tree, source)` — single DFS pass producing `Vec<Scope>`
- Types: `DefKind` (8 variants), `VarDef`, `VarUse`, `ScopeKind` (6 variants), `Scope`, `SymbolTable`
- Tracks: input/output args, assignments, for-loop iterators, global/persistent, nested functions, lambda params
- Helpers: `scope_at()`, `function_scopes()`, `defs_of()`, `uses_of()`, `is_defined()`, `is_used()`
- 14 unit tests

### 2.2 — Control Flow Analysis ✅
- **File:** `crates/mlt_rules/src/analysis/control_flow.rs` (1,448 lines)
- `find_unreachable(tree, source)` — detects code after return/break/continue
- `analyze_definite_assignment(func_node, source)` — set-based flow analysis through if/switch/try/loop branches
- `block_terminates(block_node)` — recursive termination check
- Handles: if/elseif/else intersection, switch/case/otherwise, try/catch, for/while (conservative), global/persistent as assigned
- 24 unit tests

### 2.3 — Function/Class Metadata Extraction ✅
- **File:** `crates/mlt_rules/src/analysis/metadata.rs` (1,304 lines)
- `FileMeta::build(tree, source)` — extracts `FileType`, `ClassMeta`, `FunctionMeta`, `PropertyMeta`, etc.
- Detects: file type (Script/FunctionFile/ClassFile), constructors, setters/getters, abstract methods, argument validation blocks
- Helpers: `main_function()`, `all_methods()`, `all_properties()`, `is_sealed()`, `is_abstract()`, `is_handle()`
- 10 unit tests

### 2.4 — Analysis Module Scaffold ✅
- **File:** `crates/mlt_rules/src/analysis/mod.rs`
- Re-exports `symbols`, `control_flow`, `metadata` sub-modules.

---

## Phase 3: Rule Implementation by Category ✅ COMPLETE

**Delivered via four stacked-PR waves (A–D), all merged into `feat`** (PRs #8,
#9, #10, #12), using the rule-pipeline (`.opencode/skill/rule-pipeline/`):
plan in the main session → parallel Reviewer subagents → parallel Implementer
subagents. ~185 checks landed.

### What was delivered

| Wave | Checks | Highlights |
|------|--------|------------|
| A (readability/incomplete) | 8 | COMNL, STLOW, FLUDLR, MFAMB, FVINR; QUIT (Linter panic guard), NOFIL/RDERR (CLI guards) |
| B (syntax errors) | 30 | Number-literal validation, unterminated strings/comments, reserved words, missing END/bracket variants, call/arg syntax, VTPOD, BADCT |
| C (language spec) | 108 | Parfor/SPMD (28), class/method (26), function validation (38), other (16) — all consolidated in `language_spec.rs` |
| D (good practices) | 39 | OOP/class/property (12), parfor/spmd (11), logical (3), function-call (6), structure/string (7) |

### Structural decisions that affect later phases

- **Rule IDs match MATLAB check IDs.** One engine per category, dispatching to
  per-check-ID sub-checks gated by `is_check_enabled` / `disabled_checks`.
- **Engine pattern:** most engines are hybrid — node-level `check()` for pattern
  matches plus file-level `check_file()` (using `FileMeta`/`SymbolTable`) for
  context-aware checks. See `readability.rs` (MFAMB), `good_practices.rs`.
- **`language_spec.rs` is a single consolidated module** for parfor + class +
  function-validation + other lang-spec checks (the planned
  `parfor.rs`/`class_rules.rs`/`function_validation.rs` split was not used).
- **ID renames/removals:** bugs.rs `FPFORP`/`FWFORP` → `PFTRIV`/`FWPARF`
  (the official IDs are the fprintf/fwrite language-spec checks, implemented in
  `language_spec.rs`). NOPAR → NOPAR2/EOLPAR/ENDPAR. SEPEXC is not a real
  MathWorks check. FVNST lives in `language_spec.rs`, not `syntax_errors.rs`.
- **Docs convention:** sub-checks share one engine doc page (e.g.
  `docs/language-spec.md`, `docs/good-practices.md`) instead of one page per ID.
- **Test convention:** in-module unit tests via the shared
  `crates/mlt_rules/src/test_util.rs` harness (`lint_nodes`/`lint_file`/`has_id`),
  plus CLI integration tests (`crates/mlt_cli/tests/nofil_rderr.rs`).

### Remaining Phase 3 work (~39 checks + 24 deferred)

> **Coverage:** ~730 of the full 2,680-check inventory (~27%) is implemented.
> Phase 3 in-scope categories are ~94% done; the remaining 73% of the full
> inventory is mostly Phase 4 data-file expansion (Compatibility, Behavior Changes).

| Category | Remaining |
|----------|-----------|
| Good Practices | 26 pending + 24 deferred (ADPROP, ADPROPLC, BDLGI, BDLOG1, BDLOG2, BDSCA, BDSCI, CTOINW, FXUP, GTARG, LTARG, MCCSPS, MCHDP, MCHDT, MCNPN, MCNPR, MCSUP, MCVM, MCSNOV, MCSOH, PFRIN, PFRUS, SHVAU, VTFIN) — deferred because they need type inference / cross-scope analysis |
| Language Spec | 11 (146 of 157 implemented; reconcile doc-table gap vs index count) |
| System Objects | 2 (SOTUNPROP1/3/4 consolidated into one SOTUNPROP emission; reconcile target) |

Per-category detail follows; markers (`✅ DONE` / `⚠️ partial`) reflect the
post-merge baseline.

### 3.1 — Formatting Suggestions (7 of 8) ✅ DONE
- **File:** `crates/mlt_rules/src/formatting.rs`
- **Check IDs:** NOCOMMA, NO4LP, ALIGN, NOPTS, NOPRT, PRTCAL, NCOMMA
- **Depends on:** Nothing (NOSEMI already done separately)
- **Approach:** Node-level pattern matching

| Check ID | Description | Target Nodes |
|----------|-------------|-------------|
| NOCOMMA | Missing comma between matrix elements | `matrix`, `cell` |
| NO4LP | Missing indentation for loop body | `for_statement`, `while_statement` |
| ALIGN | Misaligned code | `block` |
| NOPTS | Missing parentheses in control statement | `if_statement`, `while_statement` |
| NOPRT | Unnecessary parentheses | `parenthesis` |
| PRTCAL | Parentheses in function call could use command syntax | `function_call` |
| NCOMMA | Missing comma in function arguments | `arguments` |

### 3.2 — Bugs (35 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/bugs.rs`
- **Check IDs:** IFBDUP, IFCDUP, PFUIXE, PFBFN, PFWHOS, PFTUSE, PFRNC, BDSCA2, RHSFN, FNAN, FUNFUN, MOCUP, MDUPC, MNANC, MULCC, STCUL, LOGEMP, NOPRC, SHOCIRT, SHOCIRF, CTRUE, CFALSE, DEFSIZE, VARARG, STRCMPCSTR, MEXCEP, ASSRT, FWFORP, FPFORP, LBODUP, DEBUGFUN, INCR, DECR, CMDAND, CMDOR
- **Depends on:** 2.3 (metadata) for some checks
- **Approach:** Node-level pattern matching; some file-level

Key checks:
| Check ID | Description | Approach |
|----------|-------------|----------|
| IFBDUP | Duplicate if-branch bodies | Compare child block text |
| IFCDUP | Duplicate if-branch conditions | Compare condition node text |
| CTRUE | Condition is always true | Detect `if true`, `while true` |
| CFALSE | Condition is always false | Detect `if false` |
| DEBUGFUN | Debug function left in code | Match `keyboard`, `dbstop` calls |
| CMDAND | `&` used in command context (should be `&&`) | Check boolean_operator in command |
| CMDOR | `\|` used in command context (should be `\|\|`) | Check boolean_operator in command |
| INCR | Inefficient increment `x = x + 1` | Pattern match assignment |
| DECR | Inefficient decrement `x = x - 1` | Pattern match assignment |

### 3.3 — Readability Improvements (36 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/readability.rs`
- **Check IDs:** ASGSL, COMNL, SPERR, SPWRN, NCHKE, DSPSP, DSPSY, STLOW, FLUDLR, RPMT1, RPMT0, RPMTT, RPMTF, RPMTI, RPMTN, PSIZE, LOGSUM, LOGL, ISCHR, ISSTR, ISLOG, ISCEL, IJCL, ISMAT, ISROW, ISCOL, NBRAK2, MFAMB, FVINR, STREMP, STRCL1, STRCLFH, STRIFCND, CHARTEN, SPRINTFN
- **Depends on:** Nothing
- **Approach:** Node-level pattern matching (suggest more readable alternatives)

Key checks:
| Check ID | Description |
|----------|-------------|
| ISCHR | Use `ischar(x)` instead of `isa(x,'char')` |
| ISSTR | Use `isstring(x)` instead of `isa(x,'string')` |
| IJCL | `i` or `j` used as variable (shadows complex unit) |
| NBRAK2 | Unnecessary brackets in indexing |
| STREMP | Use `strlength(s)==0` instead of `strcmp(s,'')` |

### 3.4 — Performance Improvements (41 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/performance.rs`
- **Check IDs:** PFBNS, RGXP1, TRIM1, STTOK, TRIM2, STNCI, STCCS, FNDSB, SFLD, GFLD, CCAT, AGROW, SAGROW, ISMT, ISCL, ST2NM, FLPST, MXFND, EFIND, EXIST, UDIM, FREAD, N2UNI, TNMLP, MINV, LAXES, MMTC, MRPBW, SPRIX, TRSRT, CCAT1, GRIDD, AND2, OR2, CLALL, CLCLS, CLFUNC, CLJAVA, CLMEX, RGXPI, CLEAR0ARGS
- **Depends on:** 2.1 (symbols) for AGROW (detecting growth inside loops)
- **Approach:** Mostly node-level pattern matching

Key checks:
| Check ID | Description |
|----------|-------------|
| AGROW | Variable appears to grow inside loop |
| AND2 | Use `&&` instead of `&` for scalar logical |
| OR2 | Use `\|\|` instead of `\|` for scalar logical |
| MINV | Use `A\b` instead of `inv(A)*b` |
| GFLD | Use dynamic field names instead of `getfield` |
| SFLD | Use dynamic field names instead of `setfield` |
| EXIST | Use `isfile`/`isfolder` instead of `exist` |

### 3.5 — Good Practices (106 checks) ⚠️ 80/106 DONE, 24 deferred
- **File:** `crates/mlt_rules/src/good_practices.rs`
- **Depends on:** 2.1 (symbols), 2.3 (metadata)
- **Approach:** Mix of node-level and file-level checks

Split into sub-groups within the module:
| Sub-Group | Check IDs | Count |
|-----------|-----------|-------|
| Error handling | TRYNC, CTCH, WLAST, WNTAG, ERTAG, MEXCEP | 6 |
| String comparison | STCMP, STCI, STISA, STRNU | 4 |
| OOP practices | MCHDP, MCVM, MCPO, MCCPI, MCSUP, MOBSRV, etc. | ~20 |
| eval/dynamic code | EVLCS, EVLDOT, EVLEQ, EVLSYS, EVLDUAL, EVLSEQVAR | 6 |
| Parfor/SPMD practices | PFRNI, PFGP, PFGV, PFEVB, PFOUS, PFIIN, etc. | ~15 |
| General | NOANS, LOAD, SEPEX, NBRAK1, LNGNM, CHAIN, DISPLAY, etc. | ~55 |

### 3.6 — Incomplete Analysis (17 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/incomplete_analysis.rs` or integrated into `mlt_core/src/linter.rs`
- **Check IDs:** TMMSG, TMSMS, MXASET, QUIT, NOSPC, MBIG, NOFIL, MDOTM, MDMCR, RDERR, EOFER, EOFMI, MDEEP, DEEPC, DEEPN, DEEPS, TEXTL
- **All `can_be_disabled = false`**
- **Depends on:** Nothing (linter-internal limits)
- **Approach:** Guard conditions in the lint engine:
  - TMMSG: emit if diagnostics count exceeds 10,000
  - MBIG: emit if source file exceeds size threshold
  - MDEEP: emit if parenthesis nesting exceeds threshold
  - EOFER: emit if parse tree has too many ERROR nodes
  - NOFIL, RDERR, MDOTM: emit from CLI on file I/O errors

### 3.7 — Syntax Errors (50 checks) ✅ DONE (48 IDs; SEPEXC not real, FVNST in 3.8)
- **File:** `crates/mlt_rules/src/syntax_errors.rs`
- **Check IDs:** NOLHS, BDFIL, BADCH, BADCT, BADFP, BADHBH, BADHBB, BADHBHT, BADHBBT, BADSP, HEXTOOLONG, BINARYTOOLONG, BADOT, BADNE, DOUQT, STRIN, INBLK, RESWD, REDEF, UNSET, LHROW, NOPAR, NOPAR2, EOLPAR, TWOCM, FNDOT, ENDCT, ENDCT2, ENDCT3, ENDCT4, SYNER, SOFOC, CLIS, MCPLD, SBTMP, BADNOT, BADNOTLHS, ENDPAR, SEPEXR, SEPEXC, VTPOD, FNSWA, SYNEND, SEMFU, CLTWO, FVACI, FVACS, FVAMI, FVNST, FVSYN
- **Depends on:** 2.3 (metadata) for some
- **Approach:** Many are already caught by tree-sitter parse errors. Augment with post-parse validation:
  - Check tree-sitter `ERROR` and `MISSING` nodes for specific patterns
  - Validate file name vs class/function name (BDFIL, MCFIL)
  - Check for known bad patterns (BADNE: `!=` instead of `~=`, BADOT: `..`)

### 3.8 — Language Specification Errors (157 checks) ⚠️ 146/157 DONE
Split into sub-modules due to size:

#### 3.8a — Parfor Rules (45 checks) ✅ (in `language_spec.rs`)
- **File:** `crates/mlt_rules/src/language_spec.rs`
- **Check IDs:** PFANSLP, PFANSNS, PFFORA, PFGLOB, PFINPT, PFPERS, PFCTXT, PFFRNG, PFMLTI, PFANON, PFFSUB, PFINCR, PFVARS, PFVSUB, PFRNG, PFPF, PFSPMD, PFBRK, PFRTN, PFLD, PFSV, PFNAR, PFUTVR, PFUTMP, PFEVC, PFNAIO, PFNACK, PFSLO, PFSLW, PFSLRD, PFUNK, PFNF, PFRFH, PFXST, PFCEL, BRKFOR, CONTFOR, FWFORP, FPFORP, plus related SPMD checks
- **Depends on:** 2.1 (symbols), 2.3 (metadata)
- **Approach:** File-level; detect `for_statement` with parfor keyword, then validate variable classification

#### 3.8b — Class/Method Rules (45 checks) ✅ (in `language_spec.rs`)
- **File:** `crates/mlt_rules/src/language_spec.rs`
- **Check IDs:** MCDIR, MCFIL, MCEB, MCSGP, MCSGA, MCS2I, MCS1O, MCG1I, MCG1O, MCGSA, MCSCN, MCANI, MCASC, MCRED, MCCBD, MCPSG, MCSCT, MCSCO, MCSCF, MCCBS, MCCBU, MCCMC, MCSCC, MCSCM, MCCSOP, MTMAT, MTAGS3, MCAPP, MABSEAC, MABSEAM, MCMIO, MCMSP, MCMTP, MHERIT, MCSWA, MCPIN, MWKREF, MWKCT, MWKCL, MCSMO, plus AT* attribute checks
- **Depends on:** 2.3 (metadata)
- **Approach:** File-level; requires class structure understanding

#### 3.8c — Function Validation Rules (40 checks) ✅ (in `language_spec.rs`)
- **File:** `crates/mlt_rules/src/language_spec.rs`
- **Check IDs:** FVAPN, FVATF, FVIOA, FVBTN, FVDAN, FVDAP, FVDNF, FVDREP, FVMCL, FVNDE, FVIDV, FVNIV, FVNREP, FVOND, FVORDI, FVORDN, FVORDO, FVORDP, FVONV, FVREPD, FVREPO, FVNSC, FVNVL, FVSOR, FVSORO, FVUBD, FVVCON, FVOCON, FVVIN, FVVREP, TTOOFEWDIMS, TINVALDIM, FVOBI, FVOOD, FVOON, FVOVREP, FVOOI, FVORM, plus VTPEAL, VTPCON, VTPIN
- **Depends on:** 2.3 (metadata)
- **Approach:** File-level; validate `arguments` blocks

#### 3.8d — Other Language Spec (27 checks)
- **File:** `crates/mlt_rules/src/language_spec.rs`
- **Check IDs:** FCONV, FCONF, ROWLN, GPFST, GPNES, NPERS, SPDEC, SPDEC3, SPNST, SPRET, SPBRK, SPLD, SPSV, SPGP, SPEVC, SPBFN, SPNF, SPWHOS, FCNANS, CLANS, USESWNS, IDXCOLND, CTOINE, CTORO, NCHKOS, ERTXT, WTXT
- **Depends on:** 2.1 (symbols), 2.3 (metadata)

### 3.9 — Unset Variables (6 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/unset_variables.rs`
- **Check IDs:** PSET, USENS, SVNODEF, SUSENS, NODEF, STOUT
- **Depends on:** 2.1 (symbols), 2.2 (control flow)
- **Approach:** File-level; use symbol table to find variables read before written

### 3.10 — Unused Constructions (17 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/unused.rs`
- **Check IDs:** NOEFF, NUSED, EQEFF, PUSE, SETNU, ASGLU, NASGU, PREALL, INUSA, INUSD, VANUS, DEFNU, UNRCH, MANU, VUNUS, MSNU, MSNE
- **Depends on:** 2.1 (symbols), 2.2 (control flow)
- **Approach:** File-level; use symbol table to find unused assignments and unreachable code

### 3.11 — Suggested Improvements (243 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/suggested_improvements.rs`
- **Data:** `crates/mlt_rules/src/data/suggested_improvements.toml`
- **Depends on:** Nothing (data-driven, like compatibility)
- **Approach:** Same lookup-engine pattern as compatibility. Function name → suggested replacement.

Key checks:
| Check ID | Description |
|----------|-------------|
| CSVRD | Use `readmatrix` instead of `csvread` |
| DLMRD | Use `readmatrix` instead of `dlmread` |
| XLSRD | Use `readmatrix`/`readtable` instead of `xlsread` |
| CSVWT | Use `writematrix` instead of `csvwrite` |
| DLMWT | Use `writematrix` instead of `dlmwrite` |
| XLSWT | Use `writematrix`/`writetable` instead of `xlswrite` |
| ISDIR | Use `isfolder` instead of `isdir` |
| HIST | Use `histogram` instead of `hist` |
| HISTC | Use `histcounts` instead of `histc` |

### 3.12 — Specialized Domains ⚠️ partial (codegen 20✅, deployment 10✅, system objects 7/9, unsupported 13✅)

#### Code Generation (19 checks)
- **File:** `crates/mlt_rules/src/codegen.rs`
- **Check IDs:** EMVDF, EMGRO, EMNODEF, EMFCN, PRMNOIN, EMCEL, EMTC, EMIMP, EMNST, EMSCR, EMBRK, EMCNT, EMPFR, EMRTN, EMWHL, EMRIFAV, LOOPPRAGMAWITHOUTFOR, EMLOAD, EMS2N
- **Depends on:** 2.3 (metadata)

#### Deployment (10 checks)
- **File:** `crates/mlt_rules/src/deployment.rs`
- **Check IDs:** MCCD, MCPRD, MCHLP, MCKBD, MCSVP, MCMLR, MCABF, MCMFL, MCTBX, MCLL

#### System Objects (9 checks)
- **File:** `crates/mlt_rules/src/system_objects.rs`
- **Check IDs:** SONUMIN, SONUMOUT, SODEPPROP, SOINITPROP, SODFLTVAL, SORSRVDNM, SOTUNPROP1, SOTUNPROP3, SOTUNPROP4

#### Unsupported Features (13 checks)
- **File:** `crates/mlt_rules/src/unsupported.rs`
- **Check IDs:** MCADE, AWTIUD, AXCHUD, FEATUD, FNDPUD, HGCNUD, IMPKG, ISMBUD, MIPKG, SEPTUD, SYDEUD, UIRSUD, UISUUD

#### Fixed-Point (1 check)
- Included in `codegen.rs`: FPASE

### 3.13 — Configuration Issues (4 checks) ✅ DONE
- **File:** `crates/mlt_rules/src/config_issues.rs`
- **Check IDs:** BDCFG, CFERR, BDOPT, CFIG
- **Approach:** Emit diagnostics during config parsing for malformed `.mlt.toml`

---

## Phase 4: Expand Compatibility Data File ✅ COMPLETE

The compatibility engine (`compatibility.rs`) is code-complete and the data file
now covers the full target set. See `PHASE4_PLAN.md` for the wave record.

### 4.1 — Compatibility Considerations ✅
- **890/890 check IDs** in `data/compatibility.toml` (added 635 missing entries).
- `function_name` derived per convention (deprecated name, or parent function
  for option/property removals); ~20 property-only entries marked generic.

### 4.2 — Behavior Changes Low Reliability ✅
- **265/265 check IDs** present (JAPIEXT* + non-JAPIEXT).

### 4.3 — Upcoming Behavior Changes Low Reliability ✅
- **632/632 check IDs** present (ROSDFMISSING + 631 JAPIEXT*).

### 4.4 — Forward Compatibility ✅
- **7/7 check IDs** present (FCLEN, FCCPV, FCDQS, FCFAV, FCHBL, FCLFS, FCNVA).

### Engine & data-quality fixes
- **Deduplicated 56 duplicate IDs** (data file 1,980 → 1,924 unique entries),
  preferring entries with non-empty `function_name`.
- **Multi-function names now emit all matching diagnostics**: `COMPAT_TABLE`
  changed from `HashMap<&str, &CompatEntry>` (first-wins) to
  `HashMap<&str, Vec<&CompatEntry>>` (e.g. `tcpip` → TCPC+TCPS, `linprog` →
  LINPROGS+LINPROGD+LINPROGA, `opengl` → OPGLI/OPGLD/OPGLO).
- Remaining **68 generic entries** (`function_name = ""`) are AST-pattern checks
  the lookup engine cannot match; tracked as future work.
- Total data entries: **1,924 unique**; 1,794 target IDs fully covered.
- Tests: **1,197 passing**, zero clippy warnings.

---

## Phase 5: Test Suite ✅ DONE

The original plan described a `tests/fixtures/` directory structure. In practice the
test suite was delivered as **in-module unit tests** with a shared harness — this is
now the established pattern:

### 5.1 — Test Harness
- `crates/mlt_rules/src/test_util.rs` — `parse()`, `lint_nodes()`, `lint_file()`,
  `has_id()` helpers shared by every rule module's `#[cfg(test)]` mod.
- `crates/mlt_cli/tests/nofil_rderr.rs` — CLI-level integration tests using
  `CARGO_BIN_EXE_mlt` + `tempfile`.

### 5.2 — Test Types (implemented)
- **Unit tests:** Per-rule/per-check, in each rule module (fires, not-fires,
  config-disabled).
- **Fix tests:** Verify `Fix` byte ranges/replacements and `--fix` output.
- **Config tests:** `disabled_checks`, severity overrides, category disabling.
- **CLI integration tests:** `nofil_rderr.rs`.

Current totals: **1,195 mlt_rules + 3 mlt_cli + 10 mlt_core = 1,208 tests passing**,
with zero clippy warnings.

---

## Phase 6: Documentation

### 6.1 — Rule Documentation Pages
- One `docs/<rule_id>.md` page per non-data-driven rule (or per category for data-driven engines)
- Follow template from `docs/nosemi.md`
- Data-driven engines get a single page explaining the engine + a table of all check IDs

### 6.2 — Update `docs/rules.md`
- Complete rule table with all ~2,680 check IDs
- Sortable/filterable by category, severity, auto-fix status

### 6.3 — Update `docs/configuration.md`
- Document `[lint.categories]` feature
- Document all rule-specific parameters
- Examples for common workflows

---

## Dependency Graph & Parallelization

```
Phase 0 ✅ ─────────────────────────────────────────────────────┐
Phase 1 ✅ ─────────────────────────────────────────────────────┤
                                                                │
Phase 2 (infrastructure) ──── depends on Phase 0 ──────────────┤
  ├─ 2.1 Symbol table ─────────────→ start immediately         │
  ├─ 2.2 Control flow ─────────────→ depends on 2.1            │
  ├─ 2.3 Metadata extraction ──────→ parallel with 2.1         │
  └─ 2.4 Module scaffold ─────────→ parallel with 2.1          │
                                                                │
Phase 3 (rules) ──── depends on Phase 2 where noted ───────────┘
  │
  │  No dependencies (can start immediately):
  ├─ 3.1  Formatting (7)         ────→ PARALLEL
  ├─ 3.3  Readability (35)       ────→ PARALLEL
  ├─ 3.6  Incomplete Analysis (17) ──→ PARALLEL
  ├─ 3.11 Suggested Improvements (243) → PARALLEL (data-driven)
  ├─ 3.13 Configuration Issues (4)  ─→ PARALLEL
  │
  │  Depends on 2.3 (metadata) only:
  ├─ 3.2  Bugs (35)             ────→ PARALLEL after 2.3
  ├─ 3.7  Syntax Errors (50)    ────→ PARALLEL after 2.3
  ├─ 3.8b Class Rules (45)      ────→ PARALLEL after 2.3
  ├─ 3.8c Function Validation (40) ─→ PARALLEL after 2.3
  ├─ 3.12 Specialized (52)      ────→ PARALLEL after 2.3
  │
  │  Depends on 2.1 (symbols) + 2.3 (metadata):
  ├─ 3.4  Performance (41)      ────→ PARALLEL after 2.1+2.3
  ├─ 3.5  Good Practices (106)  ────→ PARALLEL after 2.1+2.3
  ├─ 3.8a Parfor (45)           ────→ PARALLEL after 2.1+2.3
  ├─ 3.8d Language Spec Other (27) ─→ PARALLEL after 2.1+2.3
  │
  │  Depends on 2.1 + 2.2 (symbols + control flow):
  ├─ 3.9  Unset Variables (6)   ────→ after 2.2
  └─ 3.10 Unused Constructions (17) → after 2.2

Phase 4 (data expansion) ──── no code deps, can run anytime ───
  ├─ 4.1 Compat remaining entries    → PARALLEL
  ├─ 4.2 Behavior changes entries    → PARALLEL
  ├─ 4.3 Upcoming behavior entries   → PARALLEL
  └─ 4.4 Forward compat patterns     → after engine update

Phase 5 (tests) ──── after each Phase 3 module ────────────────
Phase 6 (docs) ───── after each Phase 3 module ────────────────
```

### Recommended Execution Order

Phase 0–2 infrastructure and Phase 3 rule modules were delivered in two execution
models. **The Phase 3 waves (A–D) were executed via the stacked-PR rule-pipeline
(see the "Phase 3: Rule Implementation" section), all merged into `feat`:**

| Wave | Work Items | Status |
|------|-----------|--------|
| **Wave A** | Readability (5) + Incomplete Analysis (3) | ✅ merged (#8) |
| **Wave B** | Syntax Errors (30) | ✅ merged (#9) |
| **Wave C** | Language Spec (108) | ✅ merged (#10) |
| **Wave D** | Good Practices (39 + 24 deferred) | ✅ merged (#12) |
| **Remaining** | Good Practices (26), Language Spec (11), System Objects (2) | ⏳ next |

---

## File Structure (Final State)

```
crates/mlt_rules/src/
├── lib.rs
├── data/
│   ├── compatibility.toml            # ~323 entries (275 matchable)
│   └── suggested_improvements.toml   # 244 entries
├── analysis/
│   ├── mod.rs
│   ├── symbols.rs                    # Symbol table ✅
│   ├── control_flow.rs              # Reachability analysis ✅
│   └── metadata.rs                  # Function/class structure ✅
├── test_util.rs                      # Shared test harness ✅
├── nosemi.rs                         # ✅ NOSEMI (1 check)
├── compatibility.rs                  # ✅ Data-driven engine (~275 matchable)
├── naming.rs                         # ✅ Generic engine (81 checks)
├── custom_checks.rs                  # ✅ Metrics engine (25 checks)
├── formatting.rs                     # ✅ 7 checks
├── bugs.rs                           # ✅ 35 checks
├── readability.rs                    # ✅ 36 checks (hybrid engine)
├── performance.rs                    # ✅ 41 checks
├── good_practices.rs                 # ⚠️ 80/106 checks (24 deferred)
├── incomplete_analysis.rs            # ✅ 17 checks (QUIT/NOFIL/RDERR in linter/CLI)
├── syntax_errors.rs                  # ✅ 48 checks
├── language_spec.rs                  # ✅ 146 checks (consolidated: parfor + class + function validation + other)
├── unset_variables.rs                # ✅ 6 checks
├── unused.rs                         # ✅ 17 checks
├── suggested_improvements.rs         # ✅ 243 checks (data-driven)
├── codegen.rs                        # ✅ 20 checks
├── deployment.rs                     # ✅ 10 checks
├── system_objects.rs                 # ⚠️ 7/9 checks
└── unsupported.rs                    # ✅ 13 checks
```

---

## Verification

After each module:
```bash
cargo build          # All crates compile
cargo clippy         # Zero warnings
cargo test           # All tests pass
cargo run -- tests/fixtures/<rule>/fail.m   # Rule fires correctly
cargo run -- tests/fixtures/<rule>/pass.m   # Rule does NOT fire
```
