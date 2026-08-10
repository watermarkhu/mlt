#!/usr/bin/env python3
"""Generate mlt documentation tables and category pages from the live sources.

Two generation targets, both driven from the source of truth (Rust module doc
comments and the TOML data files):

1. **Data-driven rule-ID tables** (`render_data_driven_tables`) — the
   compatibility + suggested-improvements check tables. Executed at `zensical
   build` time via a `markdown-exec` block in `docs/rules.md`, so the tables
   always reflect the current data files.

2. **Category engine pages** (`render_engine_pages`) — one `docs/<cat>.md`
   overview page per multi-check engine, with the default severity read from
   each engine's `fn severity()`.

CLI:

    python3 tools/gen_rules_docs.py --print-data     # print data-driven tables
    python3 tools/gen_rules_docs.py --write-rules    # rewrite the rules.md section in place (idempotent)
    python3 tools/gen_rules_docs.py --write-pages    # regenerate docs/<cat>.md engine pages
    python3 tools/gen_rules_docs.py --write          # both of the above
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
RULES_MD = REPO / "docs" / "rules.md"
COMPAT_TOML = REPO / "crates" / "mlt_rules" / "src" / "data" / "compatibility.toml"
SI_TOML = REPO / "crates" / "mlt_rules" / "src" / "data" / "suggested_improvements.toml"

CATEGORY_LABEL = {
    "compatibility": "Compatibility Considerations",
    "forward-compatibility": "Forward Compatibility",
    "behavior-changes": "Behavior Changes",
}

# ---------------------------------------------------------------------------
# Data-driven tables
# ---------------------------------------------------------------------------


def parse_checks(toml_path: Path) -> list[dict]:
    """Parse [[checks]] tables from a TOML file into a list of dicts."""
    text = toml_path.read_text(encoding="utf-8")
    entries = []
    current = None
    for raw in text.splitlines():
        line = raw.strip()
        if line.startswith("[[checks]]"):
            if current is not None:
                entries.append(current)
            current = {}
        elif current is not None:
            m = re.match(r'^(id|function_name|severity|message|category|pattern)\s*=\s*(".*?"|\S+)', line)
            if m:
                key, val = m.group(1), m.group(2)
                if val.startswith('"'):
                    val = val[1:-1]
                current[key] = val
    if current is not None:
        entries.append(current)
    return entries


def table_rows(entries: list[dict]) -> str:
    if not entries:
        return "_(no checks)_\n"
    lines = ["| Check ID | Message |", "| -------- | ------- |"]
    for e in entries:
        msg = e.get("message", "").replace("|", "\\|")
        lines.append(f"| `{e['id']}` | {msg} |")
    return "\n".join(lines) + "\n"


def render_data_driven_tables() -> str:
    """Return the full 'Data-Driven Check IDs' markdown section."""
    compat_entries = parse_checks(COMPAT_TOML)
    si_entries = parse_checks(SI_TOML)

    by_category: dict[str, list[dict]] = {}
    for e in compat_entries:
        by_category.setdefault(e.get("category", "compatibility"), []).append(e)

    blocks = []
    for cat, entries in by_category.items():
        label = CATEGORY_LABEL.get(cat, cat)
        blocks.append(f"### {label} ({len(entries)} checks)\n")
        blocks.append(table_rows(entries))
    blocks.append(f"### Suggested Improvements ({len(si_entries)} checks)\n")
    blocks.append(table_rows(si_entries))

    return "## Data-Driven Check IDs\n\n" + "\n".join(blocks) + "\n"


def write_rules_md() -> int:
    """Replace the generated section of docs/rules.md in place (idempotent)."""
    generated = render_data_driven_tables()
    text = RULES_MD.read_text(encoding="utf-8")

    marker = "## Rule Categories"
    idx = text.find(marker)
    if idx == -1:
        print("ERROR: '## Rule Categories' heading not found in rules.md", file=sys.stderr)
        return 1

    section_heading = "## Data-Driven Check IDs\n\n"
    old_start = text.find(section_heading)
    if old_start != -1 and old_start < idx:
        text = text[:old_start] + text[idx:]
        idx = text.find(marker)

    new_text = text[:idx] + generated + "\n" + text[idx:]
    RULES_MD.write_text(new_text, encoding="utf-8")
    print(f"Updated {RULES_MD}")
    return 0


# ---------------------------------------------------------------------------
# Category engine pages
# ---------------------------------------------------------------------------

MODULES = {
    'bugs': 'BUGS_ENGINE',
    'performance': 'PERFORMANCE_ENGINE',
    'readability': 'READABILITY_ENGINE',
    'formatting': 'FORMATTING_ENGINE',
    'unset_variables': 'UNSET_VARIABLES_ENGINE',
    'unused': 'UNUSED_ENGINE',
    'codegen': 'CODEGEN_ENGINE',
    'deployment': 'DEPLOYMENT_ENGINE',
    'system_objects': 'SYSTEM_OBJECTS_ENGINE',
    'unsupported': 'UNSUPPORTED_ENGINE',
    'config_issues': 'CONFIG_ISSUES_ENGINE',
    'incomplete_analysis': 'INCOMPLETE_ANALYSIS',
    'syntax_errors': 'SYNTAX_ERRORS_ENGINE',
}

CATEGORY = {
    'bugs': 'Bugs', 'performance': 'Performance', 'readability': 'Readability',
    'formatting': 'Formatting', 'unset_variables': 'Unset Variables', 'unused': 'Unused Constructions',
    'codegen': 'Code Generation', 'deployment': 'Deployment', 'system_objects': 'System Objects',
    'unsupported': 'Unsupported', 'config_issues': 'Configuration Issues',
    'incomplete_analysis': 'Incomplete Analysis', 'syntax_errors': 'Syntax Errors',
}

TITLES = {
    'bugs': 'Bugs', 'performance': 'Performance Improvements', 'readability': 'Readability Improvements',
    'formatting': 'Formatting Suggestions', 'unset_variables': 'Unset Variables', 'unused': 'Unused Constructions',
    'codegen': 'MATLAB for Code Generation', 'deployment': 'MATLAB Compiler (Deployment)',
    'system_objects': 'System Objects', 'unsupported': 'Unsupported Features',
    'config_issues': 'Code Analyzer Configuration Issues', 'incomplete_analysis': 'Incomplete Analysis',
    'syntax_errors': 'Syntax Errors',
}

# Default severity per engine, mirrored from each engine's `fn severity()`.
SEVERITIES = {
    'bugs': 'Error', 'performance': 'Info', 'readability': 'Info', 'formatting': 'Info',
    'unset_variables': 'Warning', 'unused': 'Warning', 'codegen': 'Error', 'deployment': 'Warning',
    'system_objects': 'Warning', 'unsupported': 'Warning', 'config_issues': 'Error',
    'incomplete_analysis': 'Error', 'syntax_errors': 'Error',
}

# Lucide icon shown in the sidebar for each engine page.
ICONS = {
    'bugs': 'lucide/bug',
    'performance': 'lucide/zap',
    'readability': 'lucide/eye',
    'formatting': 'lucide/align-left',
    'unset_variables': 'lucide/alert-triangle',
    'unused': 'lucide/trash-2',
    'codegen': 'lucide/braces',
    'deployment': 'lucide/package',
    'system_objects': 'lucide/boxes',
    'unsupported': 'lucide/ban',
    'config_issues': 'lucide/cog',
    'incomplete_analysis': 'lucide/wrench',
    'syntax_errors': 'lucide/x-circle',
    'language_spec': 'lucide/book-marked',
    'good_practices': 'lucide/check-circle',
    'compatibility': 'lucide/archive',
    'suggested_improvements': 'lucide/lightbulb',
    'naming': 'lucide/type',
    'custom_checks': 'lucide/sliders-horizontal',
}


def extract_table(mod_dir: str) -> list[tuple[str, str]]:
    """Extract check-ID tables from the module's doc-comment table."""
    path = REPO / "crates" / "mlt_rules" / "src" / mod_dir / "mod.rs"
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except FileNotFoundError:
        return []
    rows = []
    for ln in lines:
        m = re.match(r'\s*//!\s*\|\s*`?([A-Z][A-Z0-9]{2,})`?\s*\|\s*(.+)', ln)
        if m:
            desc = m.group(2).strip()
            if desc.endswith('|'):
                desc = desc[:-1].strip()
            desc = re.sub(r'^(Error|Warning|Info)\s*\|\s*', '', desc)
            rows.append((m.group(1), desc))
    return rows


# Curated tables for modules whose doc comments lack an ID table.
CURATED = {
    'performance': [
        ('AGROW', 'Variable appears to change size on every loop iteration; consider preallocating'),
        ('SAGROW', 'Sliced variable appears to grow inside a loop'),
        ('PFBNS', 'Prefer broadcasting syntax over bsxfun calls'),
        ('RGXP1', 'Use regexp with output arguments instead of regexpi when case is known'),
        ('RGXPI', 'Use regexpi output argument form'),
        ('TRIM1', 'Use strtrim instead of deblank for trimming leading/trailing whitespace'),
        ('TRIM2', 'Prefer strtrim over deblank'),
        ('STTOK', 'Prefer split/strsplit over strtok'),
        ('STNCI', 'Use startsWith instead of comparing the first characters of a string'),
        ('STCCS', 'Use strcmp for character-vector comparison'),
        ('FNDSB', 'Prefer find(x > 0, 1) over find(x, 1) style patterns'),
        ('SFLD', 'Use dynamic field names instead of setfield'),
        ('GFLD', 'Use dynamic field names instead of getfield'),
        ('CCAT', 'Concatenate cell arrays using [] instead of extracting and reconstructing'),
        ('CCAT1', '{A{I}} can usually be replaced by A(I) or A(I)\x27'),
        ('ISMT', 'Use ismatrix instead of comparing ndims to 2'),
        ('ISCL', 'Use isscalar instead of numel(x)==1'),
        ('ST2NM', 'Prefer str2double over str2num'),
        ('FLPST', 'Prefer flip/rot90 over flipud/fliplr where equivalent'),
        ('MXFND', 'Use max with a single output when only the value is needed'),
        ('EFIND', 'Use the faster find form for simple conditions'),
        ('EXIST', 'Use isfile/isfolder instead of exist'),
        ('UDIM', 'Use numel instead of size for a single dimension when the array is 1-D'),
        ('FREAD', 'Use fread with fewer output arguments when possible'),
        ('N2UNI', 'Use unique instead of manual sort+diff patterns'),
        ('TNMLP', 'Prefer strlength over numel for strings'),
        ('MINV', 'Use A\\b instead of inv(A)*b'),
        ('LAXES', 'Prefer axes() with explicit arguments'),
        ('MMTC', 'Use mtimes/mtimesc scalar-matrix shortcuts'),
        ('MRPBW', 'Prefer repmat-avoiding broadcasting'),
        ('SPRIX', 'Use sparse indexing forms'),
        ('TRSRT', 'Use issorted instead of manual sort comparisons'),
        ('GRIDD', 'Prefer ndgrid over meshgrid where appropriate'),
        ('AND2', 'Use && instead of & for scalar logical AND'),
        ('OR2', 'Use || instead of | for scalar logical OR'),
        ('CLALL', 'Avoid clear all; it usually decreases performance'),
        ('CLCLS', 'Avoid clear classes'),
        ('CLFUNC', 'Avoid clear functions'),
        ('CLJAVA', 'Avoid clear java'),
        ('CLMEX', 'Avoid clear mex'),
        ('CLEAR0ARGS', 'clear with no arguments is often unnecessary'),
    ],
    'readability': [
        ('ASGSL', 'Assignment inside a conditional expression'),
        ('COMNL', 'Newline following comma acts as a row separator in a matrix'),
        ('SPERR', 'Prefer a message identifier for error'),
        ('SPWRN', 'Prefer a message identifier for warning'),
        ('NCHKE', 'Use narginchk/nargoutchk for argument validation'),
        ('DSPSP', 'Prefer fprintf/disp over sprintf+disp for display'),
        ('DSPSY', 'Prefer fprintf/disp over system-based display'),
        ('STLOW', 'Unnecessary UPPER/LOWER call in a comparison'),
        ('FLUDLR', 'Nested flipud(fliplr(x))/fliplr(flipud(x)) should use rot90(x, 2)'),
        ('RPMT1', 'Trivial multiplication by 1'),
        ('RPMT0', 'Trivial addition/subtraction of 0'),
        ('RPMTT', 'Boolean tautology (true || ...)'),
        ('RPMTF', 'Boolean contradiction (false && ...)'),
        ('RPMTI', 'Trivial multiplication by an identity-like expression'),
        ('RPMTN', 'Trivial negation patterns'),
        ('PSIZE', 'Use numel instead of prod(size(x))'),
        ('LOGSUM', 'Use nnz instead of sum for logical vectors'),
        ('LOGL', 'Prefer any/all over manual logical reduction'),
        ('ISCHR', 'Use ischar(x) instead of isa(x,\'char\')'),
        ('ISSTR', 'Use isstring(x) instead of isa(x,\'string\')'),
        ('ISLOG', 'Use islogical(x) instead of isa(x,\'logical\')'),
        ('ISCEL', 'Use iscell(x) instead of isa(x,\'cell\')'),
        ('IJCL', 'i or j used as a variable (shadows the complex unit)'),
        ('ISMAT', 'Use ismatrix(x) instead of ndims(x)==2'),
        ('ISROW', 'Use isrow(x) instead of size(x,1)==1'),
        ('ISCOL', 'Use iscolumn(x) instead of size(x,2)==1'),
        ('NBRAK2', 'Unnecessary brackets in indexing'),
        ('MFAMB', 'Cannot determine whether a name is a variable or function'),
        ('FVINR', 'Add an (Input) attribute to arguments blocks for readability'),
        ('STREMP', 'Use strlength(s)==0 instead of strcmp(s,\'\')'),
        ('STRCL1', 'Use strlength/strtrim instead of string-cleaning wrappers'),
        ('STRCLFH', 'Use strip instead of string-cleaning wrappers'),
        ('STRIFCND', 'Simplify if-conditions involving string comparisons'),
        ('CHARTEN', 'Use newline instead of char(10)'),
        ('SPRINTFN', 'Use num2str over simple sprintf for number formatting'),
    ],
}


def render_engine_page(mod: str, rows: list[tuple[str, str]]) -> str:
    eng = MODULES[mod]
    title = TITLES[mod]
    cat = CATEGORY[mod]
    severity = SEVERITIES.get(mod, 'Warning')
    icon = ICONS.get(mod, 'lucide/list-checks')
    lines = [
        '---',
        f'icon: {icon}',
        '---',
        '',
        f'# {title}',
        '',
        f'**Default severity:** {severity}',
        '**Auto-fix:** No',
        f'**Category:** {cat}',
        '**Can be disabled:** Yes',
        '',
        '## What this engine does',
        '',
        f'The `{eng}` rule implements the MATLAB Code Analyzer checks in the **{title}** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. `{rows[0][0] if rows else "XXXX"}`).',
        '',
        '## Check IDs',
        '',
        '| Check ID | Description |',
        '| -------- | ----------- |',
    ]
    for cid, desc in rows:
        lines.append(f'| `{cid}` | {desc.replace("|", "\\\\|")} |')
    lines += [
        '',
        '## Configuration',
        '',
        '```toml',
        f'[lint.rules.{eng}]',
    ]
    if mod in ('performance', 'codegen', 'deployment', 'system_objects', 'unsupported'):
        lines.append('skip_checks = ["AGROW"]   # Turn off specific checks')
    else:
        lines.append('disabled_checks = ["XXXX"]   # Turn off specific checks')
    lines += [
        '```',
        '',
        'See [Configuration](configuration.md#per-engine-parameters) for the full parameter list and [rules.md](rules.md) for the complete rule inventory.',
        '',
    ]
    return '\n'.join(lines)


def write_engine_pages() -> int:
    for mod in MODULES:
        rows = CURATED.get(mod) or extract_table(mod)
        seen = set()
        rows = [r for r in rows if not (r[0] in seen or seen.add(r[0]))]
        page = render_engine_page(mod, rows)
        out = REPO / "docs" / f'{mod.replace("_", "-")}.md'
        out.write_text(page, encoding="utf-8")
        print(f"wrote {out.name} ({len(rows)} checks)")
    return 0


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main() -> int:
    args = set(sys.argv[1:])
    if "--print-data" in args:
        print(render_data_driven_tables(), end="")
        return 0
    if "--write-rules" in args or "--write" in args:
        rc = write_rules_md()
        if rc:
            return rc
    if "--write-pages" in args or "--write" in args:
        rc = write_engine_pages()
        if rc:
            return rc
    if not args:
        print(__doc__)
        return 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
