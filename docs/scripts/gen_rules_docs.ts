#!/usr/bin/env bun
/**
 * Generate mlt documentation tables and category pages from the live sources.
 *
 * Drop-in replacement for the previous `tools/gen_rules_docs.py`, run with bun.
 * Two generation targets, both driven from the source of truth (Rust module doc
 * comments and the TOML data files):
 *
 * 1. **Data-driven rule-ID tables** (`render_data_driven_tables`) — the
 *    compatibility + suggested-improvements check tables. Written into the
 *    "Data-Driven Check IDs" section of `docs/rules.md`.
 *
 * 2. **Category engine pages** (`render_engine_body`) — one `docs/rules/<cat>.md`
 *    page per multi-check engine, with the default severity read from each
 *    engine's `fn severity()`.
 *
 * CLI:
 *
 *     bun docs/scripts/gen_rules_docs.ts --print-data     # print data-driven tables
 *     bun docs/scripts/gen_rules_docs.ts --write-rules    # rewrite the rules.md section in place (idempotent)
 *     bun docs/scripts/gen_rules_docs.ts --write-pages    # regenerate docs/rules/<cat>.md engine pages
 *     bun docs/scripts/gen_rules_docs.ts --write          # both of the above
 *     bun docs/scripts/gen_rules_docs.ts --check          # exit 1 if any generated file is stale
 */
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..')
const RULES_MD = join(REPO, 'docs', 'rules.md')
const RULES_DIR = join(REPO, 'docs', 'rules')
const COMPAT_TOML = join(REPO, 'crates', 'mlt_rules', 'src', 'data', 'compatibility.toml')
const SI_TOML = join(REPO, 'crates', 'mlt_rules', 'src', 'data', 'suggested_improvements.toml')

const CATEGORY_LABEL: Record<string, string> = {
  compatibility: 'Compatibility Considerations',
  'forward-compatibility': 'Forward Compatibility',
  'behavior-changes': 'Behavior Changes',
}

// ---------------------------------------------------------------------------
// Data-driven tables
// ---------------------------------------------------------------------------

interface CheckEntry {
  id?: string
  function_name?: string
  severity?: string
  message?: string
  category?: string
  pattern?: string
}

function parseChecks(tomlPath: string): CheckEntry[] {
  const text = readFileSync(tomlPath, 'utf8')
  const entries: CheckEntry[] = []
  let current: CheckEntry | null = null
  const keyval = /^(id|function_name|severity|message|category|pattern)\s*=\s*(".*?"|\S+)/
  for (const raw of text.split('\n')) {
    const line = raw.trim()
    if (line.startsWith('[[checks]]')) {
      if (current) entries.push(current)
      current = {}
    } else if (current) {
      const m = line.match(keyval)
      if (m) {
        let value = m[2]
        if (value.startsWith('"')) value = value.slice(1, -1)
        current[m[1]] = value
      }
    }
  }
  if (current) entries.push(current)
  return entries
}

function tableRows(entries: CheckEntry[]): string {
  if (!entries.length) return '_(no checks)_\n'
  const lines = ['| Check ID | Message |', '| -------- | ------- |']
  for (const entry of entries) {
    const msg = (entry.message || '').replace(/\|/g, '\\|')
    lines.push(`| \`${entry.id}\` | ${msg} |`)
  }
  return lines.join('\n') + '\n'
}

function renderTablesBody(): string {
  const compat = parseChecks(COMPAT_TOML)
  const si = parseChecks(SI_TOML)

  const byCategory = new Map<string, CheckEntry[]>()
  for (const entry of compat) {
    const cat = entry.category || 'compatibility'
    if (!byCategory.has(cat)) byCategory.set(cat, [])
    byCategory.get(cat)!.push(entry)
  }

  const parts: string[] = []
  for (const [cat, entries] of byCategory) {
    const label = CATEGORY_LABEL[cat] ?? cat
    parts.push(`### ${label} (${entries.length} checks)\n\n` + tableRows(entries))
  }
  parts.push(`### Suggested Improvements (${si.length} checks)\n\n` + tableRows(si))
  return parts.join('\n')
}

function renderDataDrivenTables(): string {
  return '## Data-Driven Check IDs\n\n' + renderTablesBody()
}

function renderGeneratedRulesSection(): string {
  const intro =
    'These tables are generated from the TOML data files by the TypeScript\n' +
    'generator in `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`)\n' +
    'to refresh them whenever `data/compatibility.toml` or\n' +
    '`data/suggested_improvements.toml` changes.\n'
  return '## Data-Driven Check IDs\n\n' + intro + '\n' + renderTablesBody()
}

function writeRulesMd(): boolean {
  const generated = renderGeneratedRulesSection()
  let text = readFileSync(RULES_MD, 'utf8')

  const marker = '## Rule Categories'
  let idx = text.indexOf(marker)
  if (idx === -1) {
    console.error("ERROR: '## Rule Categories' heading not found in rules.md")
    return false
  }

  const sectionHeading = '## Data-Driven Check IDs\n'
  let oldStart = text.indexOf(sectionHeading)
  if (oldStart !== -1 && oldStart < idx) {
    text = text.slice(0, oldStart) + text.slice(idx)
    idx = text.indexOf(marker)
  }

  const newText = text.slice(0, idx) + generated + '\n' + text.slice(idx)
  writeFileSync(RULES_MD, newText, 'utf8')
  console.log(`Updated docs/rules.md`)
  return true
}

// ---------------------------------------------------------------------------
// Category engine pages
// ---------------------------------------------------------------------------

const MODULES: Record<string, string> = {
  bugs: 'BUGS_ENGINE',
  performance: 'PERFORMANCE_ENGINE',
  readability: 'READABILITY_ENGINE',
  formatting: 'FORMATTING_ENGINE',
  unset_variables: 'UNSET_VARIABLES_ENGINE',
  unused: 'UNUSED_ENGINE',
  codegen: 'CODEGEN_ENGINE',
  deployment: 'DEPLOYMENT_ENGINE',
  system_objects: 'SYSTEM_OBJECTS_ENGINE',
  unsupported: 'UNSUPPORTED_ENGINE',
  config_issues: 'CONFIG_ISSUES_ENGINE',
  incomplete_analysis: 'INCOMPLETE_ANALYSIS',
  syntax_errors: 'SYNTAX_ERRORS_ENGINE',
}

const CATEGORY: Record<string, string> = {
  bugs: 'Bugs', performance: 'Performance', readability: 'Readability',
  formatting: 'Formatting', unset_variables: 'Unset Variables', unused: 'Unused Constructions',
  codegen: 'Code Generation', deployment: 'Deployment', system_objects: 'System Objects',
  unsupported: 'Unsupported', config_issues: 'Configuration Issues',
  incomplete_analysis: 'Incomplete Analysis', syntax_errors: 'Syntax Errors',
}

const TITLES: Record<string, string> = {
  bugs: 'Bugs', performance: 'Performance Improvements', readability: 'Readability Improvements',
  formatting: 'Formatting Suggestions', unset_variables: 'Unset Variables', unused: 'Unused Constructions',
  codegen: 'MATLAB for Code Generation', deployment: 'MATLAB Compiler (Deployment)',
  system_objects: 'System Objects', unsupported: 'Unsupported Features',
  config_issues: 'Code Analyzer Configuration Issues', incomplete_analysis: 'Incomplete Analysis',
  syntax_errors: 'Syntax Errors',
}

// Default severity per engine, mirrored from each engine's `fn severity()`.
const SEVERITIES: Record<string, string> = {
  bugs: 'Error', performance: 'Info', readability: 'Info', formatting: 'Info',
  unset_variables: 'Warning', unused: 'Warning', codegen: 'Error', deployment: 'Warning',
  system_objects: 'Warning', unsupported: 'Warning', config_issues: 'Error',
  incomplete_analysis: 'Error', syntax_errors: 'Error',
}

// Lucide icon shown in the page header for each engine page.
const ICONS: Record<string, string> = {
  bugs: 'lucide/bug',
  performance: 'lucide/zap',
  readability: 'lucide/eye',
  formatting: 'lucide/align-left',
  unset_variables: 'lucide/alert-triangle',
  unused: 'lucide/trash-2',
  codegen: 'lucide/braces',
  deployment: 'lucide/package',
  system_objects: 'lucide/boxes',
  unsupported: 'lucide/ban',
  config_issues: 'lucide/cog',
  incomplete_analysis: 'lucide/wrench',
  syntax_errors: 'lucide/x-circle',
  language_spec: 'lucide/book-marked',
  good_practices: 'lucide/check-circle',
  compatibility: 'lucide/archive',
  suggested_improvements: 'lucide/lightbulb',
  naming: 'lucide/type',
  custom_checks: 'lucide/sliders-horizontal',
}

function splitTableRow(line: string): string[] {
  const cells: string[] = []
  let current = ''
  let prevBackslash = false
  for (const ch of line) {
    if (ch === '|' && !prevBackslash) {
      cells.push(current.trim())
      current = ''
    } else {
      current += ch
    }
    prevBackslash = ch === '\\'
  }
  cells.push(current.trim())
  return cells
}

export function extractTable(modDir: string): Array<[string, string]> {
  const path = join(REPO, 'crates', 'mlt_rules', 'src', modDir, 'mod.rs')
  if (!existsSync(path)) return []
  const lines = readFileSync(path, 'utf8').split('\n')

  // Find the header row and the index of the Description column.
  let descIndex = 1
  for (const ln of lines) {
    if (!ln.includes('//!') || !ln.includes('|')) continue
    const cells = splitTableRow(ln.split('//!', 2)[1])
    if (cells.some((c) => c.includes('Check ID'))) {
      for (let i = 0; i < cells.length; i++) {
        if (cells[i].includes('Description')) {
          descIndex = i
          break
        }
      }
      break
    }
  }

  const rows: Array<[string, string]> = []
  for (const ln of lines) {
    if (!ln.includes('//!') || !ln.includes('|')) continue
    const cells = splitTableRow(ln.split('//!', 2)[1])
    if (cells.length < descIndex + 1) continue
    const idCell = cells.find((c) => /^[A-Z][A-Z0-9]{2,}$/.test(c.replace(/[` ]/g, '')))
    if (!idCell) continue
    const id = idCell.replace(/[` ]/g, '')
    let desc = cells[descIndex].trim()
    desc = desc.replace(/^(Error|Warning|Info)\s*\|?\s*/, '')
    desc = desc.replace(/\\\|/g, '|').trim()
    rows.push([id, desc])
  }
  return rows
}

// Curated tables for modules whose doc comments lack an ID table.
const CURATED: Record<string, Array<[string, string]>> = {
  performance: [
    ['AGROW', "Variable appears to change size on every loop iteration; consider preallocating"],
    ['SAGROW', 'Sliced variable appears to grow inside a loop'],
    ['PFBNS', 'Prefer broadcasting syntax over bsxfun calls'],
    ['RGXP1', 'Use regexp with output arguments instead of regexpi when case is known'],
    ['RGXPI', 'Use regexpi output argument form'],
    ['TRIM1', 'Use strtrim instead of deblank for trimming leading/trailing whitespace'],
    ['TRIM2', 'Prefer strtrim over deblank'],
    ['STTOK', 'Prefer split/strsplit over strtok'],
    ['STNCI', 'Use startsWith instead of comparing the first characters of a string'],
    ['STCCS', 'Use strcmp for character-vector comparison'],
    ['FNDSB', 'Prefer find(x > 0, 1) over find(x, 1) style patterns'],
    ['SFLD', 'Use dynamic field names instead of setfield'],
    ['GFLD', 'Use dynamic field names instead of getfield'],
    ['CCAT', 'Concatenate cell arrays using [] instead of extracting and reconstructing'],
    ['CCAT1', "{A{I}} can usually be replaced by A(I) or A(I)'"],
    ['ISMT', 'Use ismatrix instead of comparing ndims to 2'],
    ['ISCL', 'Use isscalar instead of numel(x)==1'],
    ['ST2NM', 'Prefer str2double over str2num'],
    ['FLPST', 'Prefer flip/rot90 over flipud/fliplr where equivalent'],
    ['MXFND', 'Use max with a single output when only the value is needed'],
    ['EFIND', 'Use the faster find form for simple conditions'],
    ['EXIST', 'Use isfile/isfolder instead of exist'],
    ['UDIM', 'Use numel instead of size for a single dimension when the array is 1-D'],
    ['FREAD', 'Use fread with fewer output arguments when possible'],
    ['N2UNI', 'Use unique instead of manual sort+diff patterns'],
    ['TNMLP', 'Prefer strlength over numel for strings'],
    ['MINV', 'Use A\\b instead of inv(A)*b'],
    ['LAXES', 'Prefer axes() with explicit arguments'],
    ['MMTC', 'Use mtimes/mtimesc scalar-matrix shortcuts'],
    ['MRPBW', 'Prefer repmat-avoiding broadcasting'],
    ['SPRIX', 'Use sparse indexing forms'],
    ['TRSRT', 'Use issorted instead of manual sort comparisons'],
    ['GRIDD', 'Prefer ndgrid over meshgrid where appropriate'],
    ['AND2', 'Use && instead of & for scalar logical AND'],
    ['OR2', 'Use || instead of | for scalar logical OR'],
    ['CLALL', 'Avoid clear all; it usually decreases performance'],
    ['CLCLS', 'Avoid clear classes'],
    ['CLFUNC', 'Avoid clear functions'],
    ['CLJAVA', 'Avoid clear java'],
    ['CLMEX', 'Avoid clear mex'],
    ['CLEAR0ARGS', 'clear with no arguments is often unnecessary'],
  ],
  readability: [
    ['ASGSL', 'Assignment inside a conditional expression'],
    ['COMNL', 'Newline following comma acts as a row separator in a matrix'],
    ['SPERR', 'Prefer a message identifier for error'],
    ['SPWRN', 'Prefer a message identifier for warning'],
    ['NCHKE', 'Use narginchk/nargoutchk for argument validation'],
    ['DSPSP', 'Prefer fprintf/disp over sprintf+disp for display'],
    ['DSPSY', 'Prefer fprintf/disp over system-based display'],
    ['STLOW', 'Unnecessary UPPER/LOWER call in a comparison'],
    ['FLUDLR', 'Nested flipud(fliplr(x))/fliplr(flipud(x)) should use rot90(x, 2)'],
    ['RPMT1', 'Trivial multiplication by 1'],
    ['RPMT0', 'Trivial addition/subtraction of 0'],
    ['RPMTT', 'Boolean tautology (true || ...)'],
    ['RPMTF', 'Boolean contradiction (false && ...)'],
    ['RPMTI', 'Trivial multiplication by an identity-like expression'],
    ['RPMTN', 'Trivial negation patterns'],
    ['PSIZE', 'Use numel instead of prod(size(x))'],
    ['LOGSUM', 'Use nnz instead of sum for logical vectors'],
    ['LOGL', 'Prefer any/all over manual logical reduction'],
    ["ISCHR", "Use ischar(x) instead of isa(x,'char')"],
    ["ISSTR", "Use isstring(x) instead of isa(x,'string')"],
    ["ISLOG", "Use islogical(x) instead of isa(x,'logical')"],
    ["ISCEL", "Use iscell(x) instead of isa(x,'cell')"],
    ['IJCL', 'i or j used as a variable (shadows the complex unit)'],
    ['ISMAT', 'Use ismatrix(x) instead of ndims(x)==2'],
    ['ISROW', 'Use isrow(x) instead of size(x,1)==1'],
    ['ISCOL', 'Use iscolumn(x) instead of size(x,2)==1'],
    ['NBRAK2', 'Unnecessary brackets in indexing'],
    ['MFAMB', 'Cannot determine whether a name is a variable or function'],
    ['FVINR', 'Add an (Input) attribute to arguments blocks for readability'],
    ["STREMP", "Use strlength(s)==0 instead of strcmp(s,'')"],
    ['STRCL1', 'Use strlength/strtrim instead of string-cleaning wrappers'],
    ['STRCLFH', 'Use strip instead of string-cleaning wrappers'],
    ['STRIFCND', 'Simplify if-conditions involving string comparisons'],
    ['CHARTEN', 'Use newline instead of char(10)'],
    ['SPRINTFN', 'Use num2str over simple sprintf for number formatting'],
  ],
}

function engineRows(mod: string): Array<[string, string]> {
  const rows = CURATED[mod] ?? extractTable(mod)
  const seen = new Set<string>()
  return rows.filter(([id]) => (seen.has(id) ? false : (seen.add(id), true)))
}

function renderEngineBody(mod: string, rows: Array<[string, string]>): string {
  const eng = MODULES[mod]
  const title = TITLES[mod]
  const cat = CATEGORY[mod]
  const severity = SEVERITIES[mod] ?? 'Warning'
  const example = rows[0]?.[0] ?? 'XXXX'

  const lines = [
    `# ${title}`,
    '',
    `**Default severity:** ${severity}`,
    '**Auto-fix:** No',
    `**Category:** ${cat}`,
    '**Can be disabled:** Yes',
    '',
    '## What this engine does',
    '',
    `The \`${eng}\` rule implements the MATLAB Code Analyzer checks in the **${title}** category. All checks share one engine and are dispatched by tree-sitter node kind or by file-level traversal; each diagnostic carries the specific check ID (e.g. \`${example}\`).`,
    '',
    '## Check IDs',
    '',
    '| Check ID | Description |',
    '| -------- | ----------- |',
  ]
  for (const [cid, desc] of rows) {
    lines.push(`| \`${cid}\` | ${desc.replace(/\|/g, '\\|')} |`)
  }
  lines.push(
    '',
    '## Configuration',
    '',
    '```toml',
    `[lint.rules.${eng}]`,
  )
  if (['performance', 'codegen', 'deployment', 'system_objects', 'unsupported'].includes(mod)) {
    lines.push('skip_checks = ["AGROW"]   # Turn off specific checks')
  } else {
    lines.push('disabled_checks = ["XXXX"]   # Turn off specific checks')
  }
  lines.push(
    '```',
    '',
    'See [Configuration](../configuration.md#per-engine-parameters) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.',
    '',
  )
  return lines.join('\n')
}

function enginePageName(mod: string): string {
  return mod.replace(/_/g, '-') + '.md'
}

function writeEnginePages(): boolean {
  for (const mod of Object.keys(MODULES)) {
    const icon = ICONS[mod] ?? 'lucide/list-checks'
    const content = `---\nicon: ${icon}\n---\n\n` + renderEngineBody(mod, engineRows(mod))
    const out = join(RULES_DIR, enginePageName(mod))
    writeFileSync(out, content, 'utf8')
    console.log(`wrote docs/rules/${enginePageName(mod)} (${engineRows(mod).length} checks)`)
  }
  return true
}

// ---------------------------------------------------------------------------
// --check: fail when any generated file has drifted from the sources
// ---------------------------------------------------------------------------

function checkDrift(): boolean {
  let clean = true

  const text = readFileSync(RULES_MD, 'utf8')
  const marker = '## Rule Categories'
  const idx = text.indexOf(marker)
  const sectionHeading = '## Data-Driven Check IDs\n'
  const oldStart = text.indexOf(sectionHeading)
  const actual =
    idx !== -1 && oldStart !== -1 && oldStart < idx ? text.slice(oldStart, idx) : ''
  if (actual !== renderGeneratedRulesSection() + '\n') {
    console.error('docs/rules.md generated section is out of date (run `bun run docs:gen`)')
    clean = false
  }

  for (const mod of Object.keys(MODULES)) {
    const icon = ICONS[mod] ?? 'lucide/list-checks'
    const expected = `---\nicon: ${icon}\n---\n\n` + renderEngineBody(mod, engineRows(mod))
    const out = join(RULES_DIR, enginePageName(mod))
    if (!existsSync(out) || readFileSync(out, 'utf8') !== expected) {
      console.error(
        `docs/rules/${enginePageName(mod)} is out of date (run \`bun run docs:gen\`)`,
      )
      clean = false
    }
  }

  return clean
}

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

const USAGE = `Generate mlt documentation tables and category pages.

CLI:
    bun docs/scripts/gen_rules_docs.ts --print-data     # print data-driven tables
    bun docs/scripts/gen_rules_docs.ts --write-rules    # rewrite the rules.md section in place (idempotent)
    bun docs/scripts/gen_rules_docs.ts --write-pages    # regenerate docs/rules/<cat>.md engine pages
    bun docs/scripts/gen_rules_docs.ts --write          # both of the above
    bun docs/scripts/gen_rules_docs.ts --check          # exit 1 if any generated file is stale`

function main(): number {
  const args = process.argv.slice(2)
  const has = (flag: string) => args.includes(flag)

  if (has('--print-data')) {
    process.stdout.write(renderDataDrivenTables())
    return 0
  }
  if (has('--check')) {
    return checkDrift() ? 0 : 1
  }
  // With no flags, default to writing both targets (`bun run docs:gen`).
  if (has('--help')) {
    console.log(USAGE)
    return 0
  }
  const writeRules = has('--write-rules') || has('--write') || args.length === 0
  const writePages = has('--write-pages') || has('--write') || args.length === 0
  if (writeRules && !writeRulesMd()) return 1
  if (writePages && !writeEnginePages()) return 1
  return 0
}

if (import.meta.main) {
  process.exit(main())
}
