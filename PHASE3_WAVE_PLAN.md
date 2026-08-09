# Phase 3 Wave Implementation Plan

This document is the execution plan for finishing **Phase 3: Rule Implementation
by Category** in the `mlt` MATLAB linter. It supersedes the stale per-category
sections of `PLAN.md` with an accurate, wave-based plan built on a fresh audit of
the current codebase.

Every rule is implemented via the **plan-in-main → Reviewer → Implementer** pipeline
defined in `.opencode/` (see §2). Each rule runs its own independent flow, and all
flows within a wave launch **in parallel**.

---

## 1. Current State (audited 2026-08)

Counts below were extracted directly from each rule module's registered check IDs
and cross-checked against the official MATLAB Code Analyzer check index
(`https://www.mathworks.com/help/matlab/matlab_env/index-of-code-analyzer-checks.html`,
read via Playwright; archive at `~/.local/share/opencode/tool-output/tool_fe291ac62001Qy9pI2rEjqHVAK`).

| Category | Module | Target | Implemented | Remaining |
|----------|--------|-------:|------------:|----------:|
| 3.1 Formatting | `formatting.rs` | 7 | 7 | 0 ✅ |
| 3.2 Bugs | `bugs.rs` | 35 | 35 | 0 ✅ |
| 3.3 Readability | `readability.rs` | 35 | 30 | **5** |
| 3.4 Performance | `performance.rs` | 41 | 41 | 0 ✅ |
| 3.5 Good Practices | `good_practices.rs` | 106 | 42 | **64** |
| 3.6 Incomplete Analysis | `incomplete_analysis.rs` | 17 | 14 | **3** |
| 3.7 Syntax Errors | `syntax_errors.rs` | 50 | 19 | **31** |
| 3.8 Language Spec | `language_spec.rs` | 157 | 37 | **~120** |
| 3.9 Unset Variables | `unset_variables.rs` | 6 | 6 | 0 ✅ |
| 3.10 Unused | `unused.rs` | 17 | 17 | 0 ✅ |
| 3.11 Suggested Improvements | `suggested_improvements.rs` | 243 | 243 | 0 ✅ |
| 3.12 Specialized (codegen/deployment/so/unsupported) | — | 52 | 52 | 0 ✅ |
| 3.13 Config Issues | `config_issues.rs` | 4 | 4 | 0 ✅ |
| **Total** | | **770** | **547** | **~223** |

**Remaining work = ~223 checks across 4 categories.**

---

## 2. Per-Rule Implementation Pipeline

Each rule flows through the pipeline defined in `.opencode/skill/rule-pipeline/` and
`.opencode/agent/`. **Planning happens in the main session** (it owns the single
Playwright MCP instance); review and implementation run as parallel subagent flows:

```
┌────────────────── MAIN SESSION (plans here) ──────────────────┐
│ 1. Playwright lookup on the MathWorks check index (single MCP)│
│ 2. Study codebase conventions (AGENTS.md, module, nosemi.rs)  │
│ 3. Write self-contained implementation plan                   │
└───────────────────────────────────────────────────────────────┘
        │ plan per rule
        ▼
┌──────────────────────────┐   verdict+   ┌──────────────────────────┐
│ 2. Reviewer (subagent)   │  corrected  │ 3. Implementer (subagent)│
│    mimo-v2.5             │ ──────────▶ │    deepseek-v4-flash     │
└──────────────────────────┘    plan     └──────────────────────────┘
```

| Step | Where | Model | Produces | Read-only |
|------|-------|-------|----------|-----------|
| 1 | **main session** | `opencode-go/deepseek-v4-flash` (current) | Implementation plan (rule context via Playwright from the MathWorks check index) | yes |
| 2 | `reviewer` subagent | `opencode-go/mimo-v2.5` | `VERDICT: APPROVED` / `CHANGES REQUIRED` + corrected plan | yes |
| 3 | `implementer` subagent | `opencode-go/deepseek-v4-flash` | Code, tests, docs; runs `cargo build`/`clippy`/`test` | no |

The orchestration skill (`.opencode/skill/rule-pipeline/SKILL.md`) mandates:

1. **Plan in the main session** — one plan per rule, produced up front using the
   single Playwright MCP instance (subagents never touch Playwright).
2. **Launch all Reviewers in parallel** — one per plan, with the plan text in the prompt.
3. **Launch all Implementers in parallel** — one per approved plan.

Each rule's flow is fully independent, so the wave runs stages 2 and 3 as two parallel
batches.

---

## 3. The Waves

Waves are ordered by dependency/cost. Each wave maps to one stacked-PR branch.

### Wave A — Readability + Incomplete Analysis (8 checks) → `feature/phase3-readability`

Smallest, self-contained, no dependencies on symbol tables. Good first wave to
validate the pipeline.

**A1 — Readability (5):** `crates/mlt_rules/src/readability.rs`

| Check ID | Description | Target nodes |
|----------|-------------|--------------|
| COMNL | Newline after comma in matrix row acts as row separator; suggest semicolon (or ellipsis) | `matrix` |
| STLOW | Unnecessary `upper`/`lower` call in a comparison | `function_call` |
| FLUDLR | Suggest `rot90(x,2)` instead of `flipud(fliplr(x))` / `fliplr(flipud(x))` | `function_call` |
| MFAMB | Cannot determine whether name is variable or function; assumes function | `identifier` / `function_call` |
| FVINR | Suggest adding `Input` attribute to `arguments` block | `arguments_statement` |

> Note: `readability.rs:28` module doc currently mis-describes FLUDLR as "prefer
> fullfile" — correct it while implementing.

**A2 — Incomplete Analysis (3):** `crates/mlt_rules/src/incomplete_analysis.rs` (or CLI level)

| Check ID | Description | Approach |
|----------|-------------|----------|
| QUIT | `quit`/`exit` encountered before end of file | node-level `function_call` |
| NOFIL | No output file written (linter-internal) | linter/CLI guard |
| RDERR | Input file read error | CLI guard |

These three are linter-internal guards; the flow must decide module vs
`mlt_core`/CLI placement and set `can_be_disabled = false`.

**Gate:** `cargo build` + `cargo clippy --all-targets` (zero warnings) + `cargo test`.

### Wave B — Syntax Errors (30 checks) → `feature/phase3-syntax-errors`

`crates/mlt_rules/src/syntax_errors.rs`. Many of these are parser-adjacent
validations that leverage tree-sitter `ERROR`/`MISSING` nodes.

**B — Missing 30 (SEPEXC is not a real MathWorks check; FVNST already lives in `language_spec.rs`):**
`BADCT`, `BADFP`, `BADHBH`, `BADHBB`, `BADHBHT`, `BADHBBT`, `HEXTOOLONG`,
`BINARYTOOLONG`, `DOUQT`, `STRIN`, `INBLK`, `RESWD`, `UNSET`, `LHROW`, `NOPAR2`,
`EOLPAR`, `ENDCT2`, `ENDCT3`, `ENDCT4`, `MCPLD`, `SBTMP`, `BADNOT`, `BADNOTLHS`,
`ENDPAR`, `VTPOD`, `SYNEND`, `FVACI`, `FVACS`, `FVAMI`, `FVSYN`

**Triage rule:** before planning each check, verify whether tree-sitter already
emits `ERROR`/`MISSING` for the pattern. If it does, the check is a
post-parse validator on those nodes; if not, it may be a node-level pattern check.

**Gate:** `cargo build` + `cargo clippy --all-targets` + `cargo test`.

### Wave C — Language Specification (~108 checks) → `feature/phase3-language-spec`

`crates/mlt_rules/src/language_spec.rs` (existing consolidated module — do not
split into new files unless a specific check is unwieldy). Grouped by sub-domain;
each group is a parallel batch:

**C1 — Parfor/SPMD (28):**
`FPFORP`, `FWFORP`, `PFANON`, `PFANSLP`, `PFANSNS`, `PFCEL`, `PFCTXT`, `PFEVC`,
`PFFRNG`, `PFFSUB`, `PFINCR`, `PFINPT`, `PFLD`, `PFMLTI`, `PFNACK`, `PFNAIO`,
`PFNAR`, `PFRFH`, `PFRNG`, `PFSLO`, `PFSLRD`, `PFSLW`, `PFSV`, `PFUNK`, `PFUTMP`,
`PFUTVR`, `PFVARS`, `PFVSUB`

**C2 — Class/Method (26):**
`MABSEAC`, `MABSEAM`, `MCAPP`, `MCCBS`, `MCCBU`, `MCCMC`, `MCCSOP`, `MCGSA`,
`MCMIO`, `MCMSP`, `MCMTP`, `MCPIN`, `MCPSG`, `MCSCC`, `MCSCF`, `MCSCM`, `MCSCN`,
`MCSCO`, `MCSCT`, `MCSMO`, `MCSWA`, `MTAGS3`, `MTMAT`, `MWKCL`, `MWKCT`, `MWKREF`

**C3 — Function Validation (38):**
`FVAPN`, `FVATF`, `FVBTN`, `FVDAN`, `FVDAP`, `FVDNF`, `FVDREP`, `FVIDV`, `FVIOA`,
`FVMCL`, `FVNDE`, `FVNIV`, `FVNREP`, `FVNSC`, `FVNVL`, `FVOBI`, `FVOCON`, `FVOND`,
`FVONV`, `FVOOD`, `FVOOI`, `FVOON`, `FVORDI`, `FVORDN`, `FVORDO`, `FVORDP`, `FVORM`,
`FVOVREP`, `FVREPD`, `FVREPO`, `FVSOR`, `FVSORO`, `FVUBD`, `FVVCON`, `FVVIN`,
`FVVREP`, `TINVALDIM`, `TTOOFEWDIMS`

**C4 — Other Language Spec (16):**
`CTOINE`, `CTORO`, `ERTXT`, `MHERIT`, `NCHKOS`, `SPBFN`, `SPBRK`, `SPDEC`, `SPDEC3`,
`SPEVC`, `SPLD`, `SPNF`, `SPSV`, `SPWHOS`, `USESWNS`, `WTXT`

> The four groups run as four parallel batches (each batch = many simultaneous
> per-rule flows). Depends on `analysis/metadata.rs` (2.3) for class/function checks.

**Gate:** `cargo build` + `cargo clippy --all-targets` + `cargo test`.

### Wave D — Good Practices (~64 checks) → `feature/phase3-good-practices`

`crates/mlt_rules/src/good_practices.rs`. Depends on symbol table (2.1) and
metadata (2.3). Largest remaining gap; split into sub-groups that run as parallel
batches:

**D1 — OOP practices (12, from PLAN.md list):**
`MCCPI`, `MCHDP`, `MCPO`, `MCSUP`, `MCVM`, `MOBSRV`, `PFEVB`, `PFGP`, `PFGV`, `PFIIN`,
`PFOUS`, `PFRNI`

**D2 — Remaining ~52 general checks:** enumerated from the check index during
planning; includes eval/dynamic-code (`EVL*`), error handling, string comparison,
parfor/SPMD practices, and general suggestions.

> Because Good Practices checks were not fully enumerated in `PLAN.md`, Wave D
> starts with a **planning pass in the main session**: look up each known ID on the
> MathWorks index to fill the gap list before the implementation batches run.

**Gate:** `cargo build` + `cargo clippy --all-targets` + `cargo test`.

---

## 4. Execution Protocol

### Per-rule flow (identical for every check)

1. **Plan in the main session.** Look up the rule ID's official
   description/severity/examples on the MathWorks check index via Playwright (single
   MCP instance). Read `AGENTS.md` "Adding a New Rule", the target module, `nosemi.rs`,
   and `docs/nosemi.md` as references; write a self-contained plan.
2. Launch `reviewer` subagent with the plan; returns `VERDICT` + corrected plan.
3. Launch `implementer` subagent with the approved plan; writes code/tests/docs and
   verifies with `cargo build`, `cargo clippy --all-targets`, `cargo test`.

### Batching (per wave)

- **Stage 1:** plan all rules in the main session (one plan per rule).
- **Stage 2:** spawn N reviewers (one per plan) in a single message — parallel.
- **Stage 3:** spawn N implementers (one per approved plan) — parallel.

Each rule's stages are strictly sequential; different rules never wait on each other.

### Completion criteria per rule

- Rule registered via `inventory::submit!`; `pub mod` added to `lib.rs` if new module.
- Tests added: fires, not-fires, config-respected.
- `docs/<rule_id>.md` created; row added to `docs/rules.md`; nav entry in `zensical.toml`.
- `cargo build` passes; `cargo clippy --all-targets` clean; `cargo test` passes.

### Wave gate before moving to next wave

- All rules in the wave pass the per-rule criteria.
- `PLAN.md` Current State table updated with accurate per-module counts.

---

## 5. Branch / PR Strategy

Follow the stacked-PR workflow in `PLAN.md` ("Development Workflow: Stacked Pull
Requests"). One branch per wave, stacked on `feat`:

```
feat
└── feature/phase3-readability     (Wave A, 8 checks)
    └── feature/phase3-syntax-errors    (Wave B, 31 checks)
        └── feature/phase3-language-spec    (Wave C, ~108 checks)
            └── feature/phase3-good-practices   (Wave D, ~64 checks)
```

- Use `gh stack init --base feat feature/phase3-readability feature/phase3-syntax-errors feature/phase3-language-spec feature/phase3-good-practices`.
- Commit per rule (or per sub-group), keep branches shallow (4 branches here is fine).
- Push and submit PRs bottom-up; review/merge bottom-up.
- **Do not commit during agent flows** — agents report results; a human/orchestrator
  commits per wave after the gate passes.

---

## 6. Risks & Open Items

| Risk | Mitigation |
|------|------------|
| MathWorks blocks live page fetches | Playwright (already working); fall back to local archive file |
| `function_call` doubles as indexing in tree-sitter | Planning (main session) and review must address ambiguity explicitly |
| Syntax checks already caught by tree-sitter `ERROR` nodes | Triage step in Wave B before planning |
| `FVNST` implemented in two modules | Dedupe check during Wave B triage |
| Good Practices gap list not in `PLAN.md` | Wave D opens with a planning batch to enumerate from the index |
| QUIT/NOFIL/RDERR are CLI-level, not node rules | Wave A explicitly decides module vs `mlt_core`/CLI placement |
| Parallel flows editing the same module file (`language_spec.rs`, `good_practices.rs`) | Batch implementers by sub-group; each flow owns distinct check IDs; resolve conflicts at commit time |

---

## 7. Summary

| Wave | Branch | Checks | Batch groups |
|------|--------|-------:|--------------|
| A | `feature/phase3-readability` | 8 | Readability(5) + Incomplete(3) |
| B | `feature/phase3-syntax-errors` | 30 | NumberLiterals(7) + Strings(3) + ReservedWords(5) + AssignmentLHS(2) + Brackets(3) + MissingEnd(3) + CallSyntax(5) + VTPOD(1) |
| C | `feature/phase3-language-spec` | ~108 | Parfor(28) + Class(26) + FuncVal(38) + Other(16) |
| D | `feature/phase3-good-practices` | ~64 | OOP(12) + General(~52) |
| **Total** | | **~210** | 8 parallel batches |

Completion moves mlt from **547/770 (71%)** to **770/770** for the Phase 3 rule
categories, with full docs and tests for every new rule.
