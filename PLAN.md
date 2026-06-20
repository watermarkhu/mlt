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

---

## Current State

### Implemented Engines

| Module | File | Functional Check IDs | Approach |
|--------|------|---------------------|----------|
| NOSEMI | `nosemi.rs` | 1 | Node-level, auto-fix |
| Compatibility | `compatibility.rs` + `data/compatibility.toml` | ~275 (of 323 entries; 48 are generic/unfilled) | Data-driven HashMap lookup |
| Naming | `naming.rs` | 81 (63 are no-ops without user config) | File-level traversal, configurable |
| Custom Checks | `custom_checks.rs` | 25 | File-level metrics, configurable thresholds |

**Total functional check IDs: ~382 of ~2,680 (14%)**

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
| Symbol table / scope analysis | Not started |
| Control flow analysis | Not started |
| Cross-file analysis | Not started |
| Inline suppression (`%#ok<RULE>`) | Not started |

---

## Target: MATLAB Code Analyzer Check Inventory

| # | Category | Count | Severity | Notes |
|---|----------|-------|----------|-------|
| 1 | Incomplete Analysis | 17 | Error | All `can_be_disabled = false`; linter-internal limits |
| 2 | Syntax Errors | 50 | Error | Parser-adjacent validation |
| 3 | Language Specification Errors | 157 | Error | OOP, parfor, spmd, argument validation |
| 4 | Bugs | 35 | Error | Suspicious patterns, logic errors |
| 5 | Custom Checks | 25 | Warning | **Done** — complexity/style metrics |
| 6 | Naming Checks | 81 | Info | **Done** — 9 entities × 9 check types |
| 7 | Compatibility Considerations | 891 | Error/Warning | **Partial** — 275 of 891 populated in data file |
| 8 | Forward Compatibility | 7 | Error/Warning | In data file as generic (not matchable yet) |
| 9 | Good Practices | 106 | Warning | Not started |
| 10 | Unset Variables | 6 | Warning | Needs symbol table |
| 11 | Unused Constructions | 17 | Warning/Info | Needs symbol table + control flow |
| 12 | Suggested Improvements | 243 | Info | Data-driven (function replacement suggestions) |
| 13 | Readability Improvements | 35 | Info | Pattern matching on AST |
| 14 | Formatting Suggestions | 8 | Info | **Partial** — NOSEMI done; 7 remaining |
| 15 | Performance Improvements | 41 | Info | Pattern matching on AST |
| 16 | MATLAB for Code Generation | 19 | Error | Specialized |
| 17 | Fixed-Point Messages | 1 | Warning | Specialized |
| 18 | MATLAB Compiler (Deployment) | 10 | Error/Warning | Specialized |
| 19 | System Objects | 9 | Error/Warning | Specialized |
| 20 | Unsupported Features | 13 | Warning | Specialized |
| 21 | Behavior Changes | 5 | Warning | In data file (5 entries) |
| 22 | Behavior Changes (Low Reliability) | 265 | Warning | **Partial** — ~55 of 265 in data file |
| 23 | Upcoming Behavior Changes | 3 | Warning | In data file as generic |
| 24 | Upcoming Behavior Changes (Low Reliability) | 632 | Warning | Not started |
| 25 | Code Analyzer Configuration Issues | 4 | Error | Not started |
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

## Phase 2: Semantic Analysis Infrastructure

Shared infrastructure needed by Phases 3's rule categories that require scope
analysis, variable tracking, or control flow reasoning.

### 2.1 — Symbol Table
- **New file:** `crates/mlt_rules/src/analysis/symbols.rs`
- Per-function-scope tracking:
  - Variable definitions: assignments, function args, for-loop iterators, global/persistent declarations
  - Variable reads: identifier references in expressions
  - Scope boundaries: function definitions, nested functions
  - Scope type: function, script, nested, class method
- Builds a `Vec<Scope>` from a single DFS pass over the tree.
- **Used by:** Unset Variables (6), Unused Constructions (17), some Good Practices, some Bugs

### 2.2 — Control Flow Graph (Basic)
- **New file:** `crates/mlt_rules/src/analysis/control_flow.rs`
- Basic reachability analysis:
  - Mark code after `return`, `break`, `continue` as unreachable
  - Track whether all branches of `if`/`switch` assign to a variable (definite assignment)
- **Depends on:** 2.1 (symbol table)
- **Used by:** `UNRCH`, `NODEF`, `USENS`, `STOUT`

### 2.3 — Function/Class Metadata Extraction
- **New file:** `crates/mlt_rules/src/analysis/metadata.rs`
- Single-pass extraction of:
  - Function signatures (name, input count, output count, is constructor, is setter/getter)
  - Class structure (name, superclasses, properties with attributes, methods with attributes, events, enumerations)
  - Argument validation blocks (`arguments` statements with size/class/validation functions)
  - File type (script vs function file vs class file)
- **Used by:** Language Specification Errors (157), many Good Practices, Bugs

### 2.4 — Analysis Module Scaffold
- **New file:** `crates/mlt_rules/src/analysis/mod.rs`
- Re-exports `symbols`, `control_flow`, `metadata` sub-modules.

### Dependencies
```
2.1 (symbols) ────────────────────→ can start immediately
2.2 (control_flow) ──────────────→ depends on 2.1
2.3 (metadata) ──────────────────→ can start immediately (parallel with 2.1)
```

---

## Phase 3: Rule Implementation by Category

Each category is an independent module that can be implemented by a parallel
agent. Categories are listed in priority order.

### 3.1 — Formatting Suggestions (7 remaining of 8)
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

### 3.2 — Bugs (35 checks)
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

### 3.3 — Readability Improvements (35 checks)
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

### 3.4 — Performance Improvements (41 checks)
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

### 3.5 — Good Practices (106 checks)
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

### 3.6 — Incomplete Analysis (17 checks)
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

### 3.7 — Syntax Errors (50 checks)
- **File:** `crates/mlt_rules/src/syntax_errors.rs`
- **Check IDs:** NOLHS, BDFIL, BADCH, BADCT, BADFP, BADHBH, BADHBB, BADHBHT, BADHBBT, BADSP, HEXTOOLONG, BINARYTOOLONG, BADOT, BADNE, DOUQT, STRIN, INBLK, RESWD, REDEF, UNSET, LHROW, NOPAR, NOPAR2, EOLPAR, TWOCM, FNDOT, ENDCT, ENDCT2, ENDCT3, ENDCT4, SYNER, SOFOC, CLIS, MCPLD, SBTMP, BADNOT, BADNOTLHS, ENDPAR, SEPEXR, SEPEXC, VTPOD, FNSWA, SYNEND, SEMFU, CLTWO, FVACI, FVACS, FVAMI, FVNST, FVSYN
- **Depends on:** 2.3 (metadata) for some
- **Approach:** Many are already caught by tree-sitter parse errors. Augment with post-parse validation:
  - Check tree-sitter `ERROR` and `MISSING` nodes for specific patterns
  - Validate file name vs class/function name (BDFIL, MCFIL)
  - Check for known bad patterns (BADNE: `!=` instead of `~=`, BADOT: `..`)

### 3.8 — Language Specification Errors (157 checks)
Split into sub-modules due to size:

#### 3.8a — Parfor Rules (45 checks)
- **File:** `crates/mlt_rules/src/parfor.rs`
- **Check IDs:** PFANSLP, PFANSNS, PFFORA, PFGLOB, PFINPT, PFPERS, PFCTXT, PFFRNG, PFMLTI, PFANON, PFFSUB, PFINCR, PFVARS, PFVSUB, PFRNG, PFPF, PFSPMD, PFBRK, PFRTN, PFLD, PFSV, PFNAR, PFUTVR, PFUTMP, PFEVC, PFNAIO, PFNACK, PFSLO, PFSLW, PFSLRD, PFUNK, PFNF, PFRFH, PFXST, PFCEL, BRKFOR, CONTFOR, FWFORP, FPFORP, plus related SPMD checks
- **Depends on:** 2.1 (symbols), 2.3 (metadata)
- **Approach:** File-level; detect `for_statement` with parfor keyword, then validate variable classification

#### 3.8b — Class/Method Rules (45 checks)
- **File:** `crates/mlt_rules/src/class_rules.rs`
- **Check IDs:** MCDIR, MCFIL, MCEB, MCSGP, MCSGA, MCS2I, MCS1O, MCG1I, MCG1O, MCGSA, MCSCN, MCANI, MCASC, MCRED, MCCBD, MCPSG, MCSCT, MCSCO, MCSCF, MCCBS, MCCBU, MCCMC, MCSCC, MCSCM, MCCSOP, MTMAT, MTAGS3, MCAPP, MABSEAC, MABSEAM, MCMIO, MCMSP, MCMTP, MHERIT, MCSWA, MCPIN, MWKREF, MWKCT, MWKCL, MCSMO, plus AT* attribute checks
- **Depends on:** 2.3 (metadata)
- **Approach:** File-level; requires class structure understanding

#### 3.8c — Function Validation Rules (40 checks)
- **File:** `crates/mlt_rules/src/function_validation.rs`
- **Check IDs:** FVAPN, FVATF, FVIOA, FVBTN, FVDAN, FVDAP, FVDNF, FVDREP, FVMCL, FVNDE, FVIDV, FVNIV, FVNREP, FVOND, FVORDI, FVORDN, FVORDO, FVORDP, FVONV, FVREPD, FVREPO, FVNSC, FVNVL, FVSOR, FVSORO, FVUBD, FVVCON, FVOCON, FVVIN, FVVREP, TTOOFEWDIMS, TINVALDIM, FVOBI, FVOOD, FVOON, FVOVREP, FVOOI, FVORM, plus VTPEAL, VTPCON, VTPIN
- **Depends on:** 2.3 (metadata)
- **Approach:** File-level; validate `arguments` blocks

#### 3.8d — Other Language Spec (27 checks)
- **File:** `crates/mlt_rules/src/language_spec.rs`
- **Check IDs:** FCONV, FCONF, ROWLN, GPFST, GPNES, NPERS, SPDEC, SPDEC3, SPNST, SPRET, SPBRK, SPLD, SPSV, SPGP, SPEVC, SPBFN, SPNF, SPWHOS, FCNANS, CLANS, USESWNS, IDXCOLND, CTOINE, CTORO, NCHKOS, ERTXT, WTXT
- **Depends on:** 2.1 (symbols), 2.3 (metadata)

### 3.9 — Unset Variables (6 checks)
- **File:** `crates/mlt_rules/src/unset_variables.rs`
- **Check IDs:** PSET, USENS, SVNODEF, SUSENS, NODEF, STOUT
- **Depends on:** 2.1 (symbols), 2.2 (control flow)
- **Approach:** File-level; use symbol table to find variables read before written

### 3.10 — Unused Constructions (17 checks)
- **File:** `crates/mlt_rules/src/unused.rs`
- **Check IDs:** NOEFF, NUSED, EQEFF, PUSE, SETNU, ASGLU, NASGU, PREALL, INUSA, INUSD, VANUS, DEFNU, UNRCH, MANU, VUNUS, MSNU, MSNE
- **Depends on:** 2.1 (symbols), 2.2 (control flow)
- **Approach:** File-level; use symbol table to find unused assignments and unreachable code

### 3.11 — Suggested Improvements (243 checks)
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

### 3.12 — Specialized Domains (combined, 52 checks)

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

### 3.13 — Configuration Issues (4 checks)
- Integrated into `crates/mlt_core/src/config.rs`
- **Check IDs:** BDCFG, CFERR, BDOPT, CFIG
- **Approach:** Emit diagnostics during config parsing for malformed `.mlt.toml`

---

## Phase 4: Expand Compatibility Data File

The compatibility engine (`compatibility.rs`) is code-complete. The data file
needs to be expanded from 323 entries to the full ~1,803.

### 4.1 — Compatibility Considerations (remaining ~616 entries)
- Current: 256 entries; target: 891
- Add remaining deprecated function entries from MATLAB documentation
- Handle duplicate function names (e.g., multi-function deprecations) by switching HashMap to `HashMap<String, Vec<CompatEntry>>`

### 4.2 — Behavior Changes Low Reliability (remaining ~210 entries)
- Current: ~55 entries; target: 265
- Primarily JAPIEXT* checks (Java API removals)

### 4.3 — Upcoming Behavior Changes Low Reliability (632 entries)
- Current: 0 entries; target: 632
- All JAPIEXT* entries for upcoming Java API removals

### 4.4 — Forward Compatibility (7 entries)
- Current: 7 entries but all generic (empty function_name)
- These require AST-pattern checks, not function-name lookup
- Consider implementing as node-level rules in `compatibility.rs` or a separate module

---

## Phase 5: Test Suite

### 5.1 — Test Fixture Structure
```
tests/
├── fixtures/
│   ├── nosemi/
│   │   ├── pass.m
│   │   └── fail.m
│   ├── compatibility/
│   │   ├── pass.m
│   │   └── fail.m
│   ├── naming/
│   │   ├── pass.m
│   │   └── fail.m
│   ├── custom_checks/
│   │   ├── pass.m
│   │   └── fail.m
│   ├── bugs/
│   │   ├── pass.m
│   │   └── fail.m
│   └── ... (one dir per category)
└── integration/
    ├── test_all_rules.rs
    └── test_fixes.rs
```

### 5.2 — Test Types
- **Unit tests:** Per-rule, in each rule module (using `#[cfg(test)]` mod)
- **Fixture tests:** MATLAB files with expected diagnostics; assert linter output matches
- **Fix tests:** Apply `--fix` and verify output matches expected
- **Config tests:** Verify rules respect severity overrides, category disabling, rule params

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

| Wave | Work Items | Parallelism |
|------|-----------|-------------|
| **Wave 1** | 2.1 (symbols), 2.3 (metadata), 2.4 (scaffold) | 3 agents |
| **Wave 2** | 2.2 (control flow), 3.1 (formatting), 3.3 (readability), 3.6 (incomplete), 3.11 (suggested), 3.13 (config issues) | 6 agents |
| **Wave 3** | 3.2 (bugs), 3.4 (perf), 3.5 (good practices), 3.7 (syntax), 3.8a-d (lang spec × 4), 3.12 (specialized) | 9 agents |
| **Wave 4** | 3.9 (unset vars), 3.10 (unused), 4.1-4.3 (data expansion) | 5 agents |
| **Wave 5** | Phase 5 (tests), Phase 6 (docs) | parallel per category |

---

## File Structure (Final State)

```
crates/mlt_rules/src/
├── lib.rs
├── data/
│   ├── compatibility.toml            # ~1,803 entries
│   └── suggested_improvements.toml   # ~243 entries
├── analysis/
│   ├── mod.rs
│   ├── symbols.rs                    # Symbol table
│   ├── control_flow.rs              # Reachability analysis
│   └── metadata.rs                  # Function/class structure
├── nosemi.rs                         # ✅ NOSEMI (1 check)
├── compatibility.rs                  # ✅ Data-driven engine (~1,803 checks)
├── naming.rs                         # ✅ Generic engine (81 checks)
├── custom_checks.rs                  # ✅ Metrics engine (25 checks)
├── formatting.rs                     # 7 checks
├── bugs.rs                           # 35 checks
├── readability.rs                    # 35 checks
├── performance.rs                    # 41 checks
├── good_practices.rs                 # 106 checks
├── incomplete_analysis.rs            # 17 checks
├── syntax_errors.rs                  # 50 checks
├── parfor.rs                         # 45 checks
├── class_rules.rs                    # 45 checks
├── function_validation.rs            # 40 checks
├── language_spec.rs                  # 27 checks
├── unset_variables.rs                # 6 checks
├── unused.rs                         # 17 checks
├── suggested_improvements.rs         # 243 checks
├── codegen.rs                        # 20 checks
├── deployment.rs                     # 10 checks
├── system_objects.rs                 # 9 checks
└── unsupported.rs                    # 13 checks
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
