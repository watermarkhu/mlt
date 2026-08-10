# Phase 4 Wave Implementation Plan — Expand Compatibility Data File

This document is the execution record and remaining-work plan for **Phase 4:
Expand Compatibility Data File** in the `mlt` MATLAB linter. It supersedes the
stale `PLAN.md` Phase 4 section with accurate numbers from a fresh audit.

**Status: engine is code-complete; the data file is partially populated.** The
work is **data-driven** — no rule-code changes except one engine enhancement
(duplicate-function handling). Each wave expands `data/compatibility.toml`.

---

## 1. Current State (audited 2026-08-09)

Counts below were extracted from `crates/mlt_rules/src/data/compatibility.toml`
and cross-checked against the **live** MATLAB Code Analyzer check index
(`https://www.mathworks.com/help/matlab/matlab_env/index-of-code-analyzer-checks.html`,
read via Playwright — the local archive is stale for these tables).

### Data file today

| Category (data file) | Entries | Notes |
|----------------------|--------:|-------|
| `compatibility` | 378 | 512 of the 890 target missing |
| `forward-compatibility` | 7 | target 7 ✅ complete |
| `behavior-changes` | 960 | target 897 (265 low-reliability + 632 upcoming-low) — 63 extra, needs reconciliation |
| **Total** | **1,345** | live target total **1,794** |

### Live target (authoritative, from the check index tables)

| Category | Target IDs |
|----------|-----------:|
| Compatibility Considerations (table 6) | 890 |
| Forward Compatibility (table 7) | 7 |
| Behavior Changes, Low Reliability (table 21) | 265 |
| Upcoming Behavior Changes, Low Reliability (table 23) | 632 |
| **Total** | **1,794** |

### Data quality issues found

- **512 compatibility IDs missing** (e.g. `IMPIVD`, `REDEFGI`, `NOV6`, `V6ON`,
  `INTLLEG*`, `DSP*`, `AF*`, `SESSION*`, `PRINTRAS*`, `PRINTPS*`, `SMTH*`).
- **56 duplicate IDs** in the data file (`DFCNCHK`, `CLBGEN`, `CLBBLD`,
  `CLBARY`, `COLMP`, `FDTAG`, `NSTIMP`, `IDISVARLOW`, `WEBBEHAVE`, `GLGRI`,
  …) — same ID appearing with different function names.
- **54 generic entries** (`function_name = ""`, `pattern = "generic"`) that the
  lookup engine cannot match; these need AST-pattern rules (out of scope here,
  tracked separately).
- **Behavior-changes over-counted by 63** — the 960 entries exceed the 897
  target (265 + 632), so some entries are stale or mis-categorized (e.g.
  duplicates, or entries that belong to other categories).
- **`JAPIEXT*` split**: 923 of the 960 behavior entries are `JAPIEXT*` — these
  map to the 265 low-reliability + 632 upcoming-low tables; needs verification.

### Engine note

`compatibility.rs` uses `HashMap<&str, &CompatEntry>` where duplicate
function names drop to the first entry (e.g. `tcpip` → only `TCPC`, not `TCPS`).
This is the pre-existing known issue.

---

## 2. Execution Approach

Because this is **data-file expansion**, the pipeline differs from Phase 3:

- **Plan in the main session** — done here (audit above).
- **Data extraction in the main session** — the check index is read via
  Playwright (single MCP instance); each wave extracts a batch of target
  check IDs + their official messages.
- **Parallel implementer subagents** — one per wave batch, each responsible for
  **adding TOML entries** to `data/compatibility.toml` for its assigned check
  IDs, with the correct `function_name`, `severity`, `message`, and `category`.
- **Verification per wave** — `cargo build` (TOML must parse), a unit test that
  asserts the new check IDs resolve, and `cargo test`.

### Data-entry template

```toml
[[checks]]
id = "XXXX"
function_name = "somefunction"
severity = "warning"        # "error" or "warning" (per MATLAB metadata)
message = "'somefunction' has been removed. Use 'replacement' instead."
category = "compatibility"  # "compatibility" | "forward-compatibility" | "behavior-changes"
```

For **JAPIEXT\*** entries the message template is uniform (Java API removal);
extract the per-ID message from the check index during planning.

---

## 3. The Waves

### Wave 1 — Compatibility Considerations, Part 1 (~256 IDs) → branch `phase4/compat-1`

Add ~256 of the 512 missing compatibility IDs. Batched alphabetically or by
toolbox family (e.g. `DSP*`/`AF*` signal-processing cluster first — they share
message templates). Includes the DSP filter bank, adaptive-filter, and
`SESSION*` families.

**Files:** `crates/mlt_rules/src/data/compatibility.toml` (append entries).

**Gate:** `cargo build` (TOML parse) + `cargo test -p mlt_rules compatibility`.

### Wave 2 — Compatibility Considerations, Part 2 (~256 IDs) → `phase4/compat-2`

Remaining compatibility IDs (e.g. `PRINTRAS*`, `PRINTPS*`, `SMTH*`, mapping
toolbox, OPC, GPIB, legacy plot/3D families).

**Gate:** same as Wave 1.

### Wave 3 — Behavior Changes reconciliation + JAPIEXT (~240 entries) → `phase4/behavior`

Resolve the 63-entry over-count and the 923 `JAPIEXT*` entries:
- Split `JAPIEXT*` between the 265 low-reliability and 632 upcoming-low targets
  correctly (the data file currently lumps them).
- Remove stale/duplicate entries so the behavior-changes set matches 897.
- Dedupe the 56 duplicate IDs (keep one entry per ID; merge function names into
  a single entry with the correct function).

**Gate:** `cargo build` + `cargo test -p mlt_rules compatibility` + a new
dedup test.

### Wave 4 — Duplicate-function engine fix → `phase4/engine-dup`

Enhance `compatibility.rs` so a `function_name` mapping to multiple check IDs
emits **all** of them (switch to `HashMap<&str, Vec<&CompatEntry>>`). Required
for correctness once `tcpip`, `legend`, `web`-style multi-checks are fully
populated. Add a unit test.

**Files:** `crates/mlt_rules/src/compatibility.rs`, tests.

**Gate:** `cargo build` + `cargo clippy --all-targets` (zero warnings) +
`cargo test`.

### Wave 5 — Completion check + PLAN.md update → `phase4/complete`

- Re-run the gap analysis against the live index; confirm 890 compat + 7
  forward + 265 behavior-low + 632 upcoming-low are all present.
- Update `PLAN.md` inventory rows (7, 8, 21–24) and the Phase 4 section.
- Update `docs/rules.md` / `zensical.toml` only if the data-driven engine page
  needs count updates.

**Gate:** full `cargo build` + `cargo clippy --all-targets` + `cargo test`.

---

## 4. Verification

Per wave:
```bash
cargo build                          # TOML must parse; engine compiles
cargo clippy --all-targets           # zero warnings (Waves 4-5)
cargo test -p mlt_rules compatibility # data-resolution tests pass
cargo test                           # full suite unaffected
```

Completion criteria:
- All 1,794 target IDs present in `data/compatibility.toml`.
- No duplicate IDs; every ID has a non-empty `function_name` unless it is a
  documented generic (AST-pattern) check.
- Engine emits multiple diagnostics for multi-function names.
- `PLAN.md` updated to reflect the completed state.

---

## 5. Branch / PR Strategy

Use stacked PRs on `feat` as before:

```
feat
└── phase4/compat-1      (Wave 1)
    └── phase4/compat-2      (Wave 2)
        └── phase4/behavior     (Wave 3)
            └── phase4/engine-dup  (Wave 4)
                └── phase4/complete  (Wave 5)
```

Or a single branch per data-expansion wave given data-file edits are low-risk.
Use `gh stack init --base feat phase4/compat-1 ...` and submit bottom-up.

---

## 6. Risks & Open Items

| Risk | Mitigation |
|------|------------|
| Live index is large (1,794 rows); Playwright extraction is slow | Batch by table; extract once, save to a working file |
| TOML parse failure on large append | `cargo build` gate after each wave; entries validated by the engine's `LazyLock` parse |
| JAPIEXT* message uniformity | Extract exact per-ID messages during planning; use templates for families |
| Duplicate function names shadow checks | Wave 4 engine fix (`Vec<CompatEntry>`) |
| 54 generic (AST-pattern) checks unmatchable | Track separately; out of Phase 4 scope |
| `function_call` doubles as indexing | No impact — lookup is by name only, like the existing engine |

---

## 7. Summary

| Wave | Branch | Scope | Entries |
|------|--------|-------|--------:|
| 1 | `phase4/compat-1` | Compatibility part 1 | ~256 |
| 2 | `phase4/compat-2` | Compatibility part 2 | ~256 |
| 3 | `phase4/behavior` | Behavior-changes reconcile + JAPIEXT | ~240 (net) |
| 4 | `phase4/engine-dup` | Multi-function duplicate fix | engine code |
| 5 | `phase4/complete` | Gap check + docs | — |

Completing these waves moves the compatibility/behavior categories from
**1,345 data entries** to the full **1,794-check target**, fixing duplicate-ID
and multi-function-emission issues along the way.
