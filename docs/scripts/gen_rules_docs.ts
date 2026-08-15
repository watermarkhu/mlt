#!/usr/bin/env bun
/**
 * Generate mlt documentation pages from the rule docstring schema.
 *
 * Every rule module in `crates/mlt_rules/src` documents itself with a
 * standardized `//!` header docstring (see `docs/scripts/SCHEMA.md`). This
 * script reads those docstrings and emits:
 *
 * 1. **Rule pages** (`docs/rules/<slug>.md`) — one page per rule module, with
 *    metadata badges, a `## Check IDs` anchor list, and the docstring's
 *    `Rule` / `Fix` / `Examples` sections.
 * 2. **The rule index table** in `docs/rules.md` — one row per check ID,
 *    linking back to its rule page (and per-check anchor where available).
 * 3. **Data-driven check tables** — the compatibility + suggested-improvements
 *    inventories from the TOML data files.
 *
 * Modules that have NOT yet been migrated to the schema are rendered through a
 * legacy fallback so the docs stay consistent during migration.
 *
 * CLI:
 *
 *     bun docs/scripts/gen_rules_docs.ts --print-data     # print data-driven tables
 *     bun docs/scripts/gen_rules_docs.ts --write          # write rule pages + rules.md (default)
 *     bun docs/scripts/gen_rules_docs.ts --check          # exit 1 if any generated file is stale
 */
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..')
const RULES_MD = join(REPO, 'docs', 'rules.md')
const RULES_DIR = join(REPO, 'docs', 'rules')
const SRCS = join(REPO, 'crates', 'mlt_rules', 'src')
const DATA_DIR = join(SRCS, 'data')
const COMPAT_TOML = join(DATA_DIR, 'compatibility.toml')
const SI_TOML = join(DATA_DIR, 'suggested_improvements.toml')

// ---------------------------------------------------------------------------
// Category metadata (human labels + default icons)
// ---------------------------------------------------------------------------

const CATEGORY_HUMAN: Record<string, string> = {
  bugs: 'Bugs',
  formatting: 'Formatting',
  performance: 'Performance',
  readability: 'Readability',
  'unset-variables': 'Unset Variables',
  'unused-constructions': 'Unused Constructions',
  'code-generation': 'Code Generation',
  deployment: 'Deployment',
  'system-objects': 'System Objects',
  unsupported: 'Unsupported',
  'configuration-issues': 'Configuration Issues',
  'incomplete-analysis': 'Incomplete Analysis',
  'syntax-errors': 'Syntax Errors',
  'language-specification': 'Language Specification',
  'good-practices': 'Good Practices',
  compatibility: 'Compatibility',
  'forward-compatibility': 'Forward Compatibility',
  'behavior-changes': 'Behavior Changes',
  'suggested-improvements': 'Suggested Improvements',
  naming: 'Naming',
  'custom-checks': 'Custom Checks',
  'fixed-point': 'Fixed-Point',
}

const CATEGORY_ICON: Record<string, string> = {
  bugs: 'lucide/bug',
  performance: 'lucide/zap',
  readability: 'lucide/eye',
  formatting: 'lucide/align-left',
  'unset-variables': 'lucide/alert-triangle',
  'unused-constructions': 'lucide/trash-2',
  'code-generation': 'lucide/braces',
  deployment: 'lucide/package',
  'system-objects': 'lucide/boxes',
  unsupported: 'lucide/ban',
  'configuration-issues': 'lucide/cog',
  'incomplete-analysis': 'lucide/wrench',
  'syntax-errors': 'lucide/x-circle',
  'language-specification': 'lucide/book-marked',
  'good-practices': 'lucide/check-circle',
  compatibility: 'lucide/archive',
  'suggested-improvements': 'lucide/lightbulb',
  naming: 'lucide/type',
  'custom-checks': 'lucide/sliders-horizontal',
}

const SEVERITY_LABEL: Record<string, string> = {
  error: 'Error',
  warning: 'Warning',
  info: 'Info',
}

// ---------------------------------------------------------------------------
// Docstring extraction
// ---------------------------------------------------------------------------

/** Return the leading `//!` module docstring of a file as markdown text. */
function moduleDocstring(file: string): string {
  const lines = readFileSync(file, 'utf8').split('\n')
  const out: string[] = []
  for (const ln of lines) {
    const m = ln.match(/^\s*\/\/!(.*)$/)
    if (!m) break
    out.push(m[1].replace(/^ /, ''))
  }
  return out.join('\n')
}

interface FrontMatter {
  id: string
  title: string
  category: string
  severity: string
  fix: boolean
  icon?: string
  slug?: string
  data_file?: string
  note?: string
  generated?: string
  [k: string]: unknown
}

/** Parse the ```mlt front-matter block from a docstring, if present. */
function parseFrontMatter(doc: string): FrontMatter | null {
  const lines = doc.split('\n')
  let i = lines.findIndex((l) => l.trim() === '```mlt')
  if (i === -1) return null
  const kv: Record<string, string> = {}
  for (let j = i + 1; j < lines.length; j++) {
    const l = lines[j].trim()
    if (l === '```') break
    const m = l.match(/^([A-Za-z_]+)\s*=\s*(.+)$/)
    if (!m) continue
    let v = m[2].trim()
    if ((v.startsWith('"') && v.endsWith('"')) || (v.startsWith("'") && v.endsWith("'"))) {
      v = v.slice(1, -1)
    }
    kv[m[1]] = v
  }
  if (!kv.id) return null
  return {
    id: kv.id,
    title: kv.title ?? kv.id,
    category: kv.category ?? '',
    severity: kv.severity ?? 'warning',
    fix: (kv.fix ?? 'false') === 'true',
    icon: kv.icon,
    slug: kv.slug,
    data_file: kv.data_file,
    note: kv.note,
    generated: kv.generated,
    ...kv,
  }
}

interface Section {
  heading: string
  body: string[]
}

/** Split the docstring (after the front-matter block) into `##` sections. */
function splitSections(doc: string): Section[] {
  const lines = doc.split('\n')
  const fmEnd = lines.findIndex((l) => l.trim() === '```mlt')
  let start = 0
  if (fmEnd !== -1) {
    start = fmEnd
    for (let j = fmEnd + 1; j < lines.length; j++) {
      if (lines[j].trim() === '```') {
        start = j + 1
        break
      }
    }
  }
  const sections: Section[] = []
  let cur: Section | null = null
  for (let j = start; j < lines.length; j++) {
    const l = lines[j]
    if (l.startsWith('## ')) {
      if (cur) sections.push(cur)
      cur = { heading: l.slice(3).trim(), body: [] }
    } else if (cur) {
      cur.body.push(l)
    }
  }
  if (cur) sections.push(cur)
  return sections
}

function sectionBody(sections: Section[], heading: string): string | null {
  const s = sections.find((x) => x.heading === heading)
  if (!s) return null
  const text = s.body.join('\n').replace(/^\n+|\n+$/g, '')
  return text.length ? text : null
}

interface CheckRow {
  id: string
  severity: string
  fix: boolean
  description: string
  category?: string
}

/** Parse a markdown `| Check ID | Severity | Fix | Description |` table. */
function parseCheckTable(lines: string[]): CheckRow[] {
  const rows: CheckRow[] = []
  let inHeader = false
  for (const raw of lines) {
    const l = raw.trim()
    if (!l.startsWith('|')) continue
    let cells = splitTableRow(l)
    // Drop the leading empty cell produced by the leading `|`.
    if (cells.length && cells[0] === '') cells = cells.slice(1)
    if (cells.some((c) => c.includes('Check ID'))) {
      inHeader = true
      continue
    }
    if (inHeader && cells.every((c) => /^:?-{2,}:?$/.test(c.trim()))) continue
    if (!inHeader) continue
    const id = (cells[0] ?? '').replace(/[` ]/g, '')
    if (!/^[A-Za-z][A-Za-z0-9._]*$/.test(id)) continue
    const severity = (cells[1] ?? 'warning').trim().toLowerCase()
    const fix = (cells[2] ?? 'no').trim().toLowerCase().startsWith('y')
    const description = (cells[3] ?? '').trim()
    rows.push({ id, severity, fix, description })
  }
  return rows
}

// ---------------------------------------------------------------------------
// Data file parsing (data-driven engines)
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

// ---------------------------------------------------------------------------
// Module discovery
// ---------------------------------------------------------------------------

interface RuleModule {
  file: string
  doc: string
  frontmatter: FrontMatter
  sections: Section[]
  checks: CheckRow[]
  description: string // from `fn description()` in source
}

/** Extract the rule description string literal from `fn description()`. */
function extractDescription(file: string): string {
  const text = readFileSync(file, 'utf8')
  const m = text.match(/fn\s+description\s*\([^)]*\)\s*->\s*&'static\s+str\s*\{\s*"([^"]*)"/)
  return m ? m[1] : ''
}

function walkRs(dir: string, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry)
    const st = statSync(p)
    if (st.isDirectory()) walkRs(p, out)
    else if (entry.endsWith('.rs')) out.push(p)
  }
  return out
}

/** Discover all modules whose docstring carries the `mlt` front-matter. */
function discoverModules(): RuleModule[] {
  const modules: RuleModule[] = []
  for (const file of walkRs(SRCS)) {
    const doc = moduleDocstring(file)
    const frontmatter = parseFrontMatter(doc)
    if (!frontmatter) continue
    const sections = splitSections(doc)
    const checks = loadChecks(frontmatter, sections)
    modules.push({
      file,
      doc,
      frontmatter,
      sections,
      checks,
      description: extractDescription(file),
    })
  }
  return modules
}

function loadChecks(fm: FrontMatter, sections: Section[]): CheckRow[] {
  if (fm.generated === 'naming') {
    return namingChecks().map(([id, desc]) => ({
      id,
      severity: 'info',
      fix: false,
      description: desc,
    }))
  }
  if (fm.data_file) {
    const path = join(DATA_DIR, fm.data_file)
    if (!existsSync(path)) {
      console.error(`ERROR: data file not found: ${path}`)
      return []
    }
    return parseChecks(path).map((e) => ({
      id: e.id ?? '',
      severity: e.severity ?? 'warning',
      fix: false,
      description: e.message ?? '',
      category: e.category,
    }))
  }
  const table = sectionBody(sections, 'Check IDs')
  if (table) {
    return parseCheckTable(table.split('\n'))
  }
  return []
}

/** VitePress heading anchor (lowercased, non-alphanumerics removed). */
export function anchor(id: string): string {
  return id.toLowerCase().replace(/[^a-z0-9]+/g, '')
}

// ---------------------------------------------------------------------------
// Rule page rendering (schema path)
// ---------------------------------------------------------------------------

function renderRulePage(mod: RuleModule): string {
  const fm = mod.frontmatter
  const slug = fm.slug ?? fm.id.toLowerCase().replace(/_/g, '-')
  const icon = fm.icon ?? CATEGORY_ICON[fm.category] ?? 'lucide/list-checks'
  const categoryLabel = CATEGORY_HUMAN[fm.category] ?? fm.category
  const severityLabel = SEVERITY_LABEL[fm.severity] ?? fm.severity

  const out: string[] = []
  out.push('---', `icon: ${icon}`, '---', '')
  out.push(`# ${fm.title}`, '')
  out.push(`**Default severity:** ${severityLabel}`)
  out.push(`**Auto-fix:** ${fm.fix ? 'Yes' : 'No'}`)
  out.push(`**Category:** ${categoryLabel}`)
  out.push('**Can be disabled:** Yes')
  if (fm.note) out.push('', `> *${fm.note}*`)
  out.push('')

  const rule = sectionBody(mod.sections, 'Rule')
  if (rule) {
    out.push('## What this rule does', '', rule, '')
  }

  if (mod.checks.length) {
    out.push('## Check IDs', '')
    const isSmall = mod.checks.length <= 60
    if (isSmall) {
      for (const c of mod.checks) {
        out.push(`### ${c.id}`, '')
        out.push(
          `Severity: **${c.severity}** · Auto-fix: **${c.fix ? 'yes' : 'no'}**`,
          '',
          c.description,
          '',
        )
      }
    } else {
      // Large data-driven engines: compact table, anchored ID cells.
      const byCategory = new Map<string, CheckRow[]>()
      for (const c of mod.checks) {
        const cat = c.category ?? fm.category
        if (!byCategory.has(cat)) byCategory.set(cat, [])
        byCategory.get(cat)!.push(c)
      }
      for (const [cat, entries] of byCategory) {
        const label = CATEGORY_HUMAN[cat] ?? cat
        out.push(`### ${label} (${entries.length} checks)`, '')
        out.push(
          '| Check ID | Severity | Description |',
          '| -------- | -------- | ----------- |',
        )
        for (const c of entries) {
          const desc = c.description.replace(/\|/g, '\\|')
          out.push(`| <a id="${anchor(c.id)}"></a>\`${c.id}\` | ${c.severity} | ${desc} |`)
        }
        out.push('')
      }
    }
  }

  if (fm.fix) {
    const fix = sectionBody(mod.sections, 'Fix')
    if (fix) {
      out.push('## Automatic fixes', '', fix, '')
    }
  }

  const examples = sectionBody(mod.sections, 'Examples')
  if (examples) {
    out.push('## Examples', '', examples, '')
  }

  const config = sectionBody(mod.sections, 'Configuration')
  if (config) {
    out.push('## Configuration', '', config, '')
  }

  out.push(
    'See [Configuration](../configuration.md) for the full parameter list and [rules.md](../rules.md) for the complete rule inventory.',
    '',
  )
  return out.join('\n')
}

function rulePageName(mod: RuleModule): string {
  const slug = mod.frontmatter.slug ?? mod.frontmatter.id.toLowerCase().replace(/_/g, '-')
  return slug + '.md'
}

// ---------------------------------------------------------------------------
// Legacy fallback for un-migrated engines
// ---------------------------------------------------------------------------

const LEGACY_MODULES: Record<string, string> = {
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

const LEGACY_TITLES: Record<string, string> = {
  performance: 'Performance Improvements',
  readability: 'Readability Improvements',
  formatting: 'Formatting Suggestions',
  unset_variables: 'Unset Variables',
  unused: 'Unused Constructions',
  codegen: 'MATLAB for Code Generation',
  deployment: 'MATLAB Compiler (Deployment)',
  system_objects: 'System Objects',
  unsupported: 'Unsupported Features',
  config_issues: 'Code Analyzer Configuration Issues',
  incomplete_analysis: 'Incomplete Analysis',
  syntax_errors: 'Syntax Errors',
}

const LEGACY_SEVERITIES: Record<string, string> = {
  performance: 'Info',
  readability: 'Info',
  formatting: 'Info',
  unset_variables: 'Warning',
  unused: 'Warning',
  codegen: 'Error',
  deployment: 'Warning',
  system_objects: 'Warning',
  unsupported: 'Warning',
  config_issues: 'Error',
  incomplete_analysis: 'Error',
  syntax_errors: 'Error',
}

const LEGACY_ICONS: Record<string, string> = {
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
}

const LEGACY_CATEGORY: Record<string, string> = {
  performance: 'Performance',
  readability: 'Readability',
  formatting: 'Formatting',
  unset_variables: 'Unset Variables',
  unused: 'Unused Constructions',
  codegen: 'Code Generation',
  deployment: 'Deployment',
  system_objects: 'System Objects',
  unsupported: 'Unsupported',
  config_issues: 'Configuration Issues',
  incomplete_analysis: 'Incomplete Analysis',
  syntax_errors: 'Syntax Errors',
}

/** Curated check tables for modules whose doc comments lack an ID table. */
const LEGACY_CURATED: Record<string, Array<[string, string]>> = {
  performance: [
    ['AGROW', 'Variable appears to change size on every loop iteration; consider preallocating'],
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
    ['ISCHR', "Use ischar(x) instead of isa(x,'char')"],
    ['ISSTR', "Use isstring(x) instead of isa(x,'string')"],
    ['ISLOG', "Use islogical(x) instead of isa(x,'logical')"],
    ['ISCEL', "Use iscell(x) instead of isa(x,'cell')"],
    ['IJCL', 'i or j used as a variable (shadows the complex unit)'],
    ['ISMAT', 'Use ismatrix(x) instead of ndims(x)==2'],
    ['ISROW', 'Use isrow(x) instead of size(x,1)==1'],
    ['ISCOL', 'Use iscolumn(x) instead of size(x,2)==1'],
    ['NBRAK2', 'Unnecessary brackets in indexing'],
    ['MFAMB', 'Cannot determine whether a name is a variable or function'],
    ['FVINR', 'Add an (Input) attribute to arguments blocks for readability'],
    ['STREMP', "Use strlength(s)==0 instead of strcmp(s,'')"],
    ['STRCL1', 'Use strlength/strtrim instead of string-cleaning wrappers'],
    ['STRCLFH', 'Use strip instead of string-cleaning wrappers'],
    ['STRIFCND', 'Simplify if-conditions involving string comparisons'],
    ['CHARTEN', 'Use newline instead of char(10)'],
    ['SPRINTFN', 'Use num2str over simple sprintf for number formatting'],
  ],
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
  const candidates = [join(SRCS, modDir, 'mod.rs'), join(SRCS, `${modDir}.rs`)]
  const path = candidates.find((p) => existsSync(p))
  if (!path) return []
  const lines = readFileSync(path, 'utf8').split('\n')

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

function legacyRows(mod: string): Array<[string, string]> {
  const rows = LEGACY_CURATED[mod] ?? extractTable(mod)
  const seen = new Set<string>()
  return rows.filter(([id]) => (seen.has(id) ? false : (seen.add(id), true)))
}

function legacyPageName(mod: string): string {
  return mod.replace(/_/g, '-') + '.md'
}

function renderLegacyBody(mod: string, rows: Array<[string, string]>): string {
  const eng = LEGACY_MODULES[mod]
  const title = LEGACY_TITLES[mod]
  const cat = LEGACY_CATEGORY[mod]
  const severity = LEGACY_SEVERITIES[mod] ?? 'Warning'
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

// ---------------------------------------------------------------------------
// rules.md — rule index table
// ---------------------------------------------------------------------------

/** Hand-written category pages not yet migrated to the schema. */
const LEGACY_CATALOG: Record<
  string,
  { page: string; category: string; severity: string; checks: Array<[string, string]> }
> = {
  nosemi: {
    page: 'rules/nosemi.md',
    category: 'formatting',
    severity: 'info',
    checks: [['NOSEMI', 'Statement without trailing semicolon may produce unintended console output']],
  },
  'good-practices': {
    page: 'rules/good-practices.md',
    category: 'good-practices',
    severity: 'warning',
    checks: extractTable('good_practices'),
  },
  'language-spec': {
    page: 'rules/language-spec.md',
    category: 'language-specification',
    severity: 'error',
    checks: extractTable('language_spec'),
  },
  'custom-checks': {
    page: 'rules/custom-checks.md',
    category: 'custom-checks',
    severity: 'warning',
    checks: extractTable('custom_checks'),
  },
}

/** Naming engine check IDs: `naming.<entity>.<checkType>` Cartesian product. */
function namingChecks(): Array<[string, string]> {
  const entities = [
    'class',
    'function',
    'localFunction',
    'method',
    'nestedFunction',
    'property',
    'event',
    'enumeration',
    'variable',
  ]
  const checkTypes = [
    'maxLength',
    'minLength',
    'regularExpression',
    'requiredPrefix',
    'disallowedPrefix',
    'disallowedPhrase',
    'requiredSuffix',
    'disallowedSuffix',
    'casing',
  ]
  const out: Array<[string, string]> = []
  for (const e of entities) {
    for (const t of checkTypes) {
      out.push([`naming.${e}.${t}`, `Naming check for ${e} ${t}`])
    }
  }
  return out
}

function catalogRows(): Array<[string, string, string, string, string]> {
  // [id, page, category, severity, description]
  const rows: Array<[string, string, string, string, string]> = []
  for (const spec of Object.values(LEGACY_CATALOG)) {
    for (const [id, desc] of spec.checks) {
      rows.push([id, spec.page, spec.category, spec.severity, desc])
    }
  }
  for (const [id, desc] of namingChecks()) {
    rows.push([id, 'rules/naming.md', 'naming', 'info', desc])
  }
  return rows
}

/** Row in the global rule index: links back to a rule page + anchor. */
function renderRuleTable(modules: RuleModule[]): string {
  const rows: string[] = [
    '| Check ID | Category | Description | Default Severity | Auto-fix |',
    '| -------- | -------- | ----------- | ---------------- | -------- |',
  ]
  const seen = new Set<string>()
  const push = (id: string, link: string, category: string, desc: string, severity: string, fix: boolean) => {
    if (seen.has(id) || !id) return
    seen.add(id)
    const cat = CATEGORY_HUMAN[category] ?? category
    rows.push(
      `| [\`${id}\`](${link}) | ${cat} | ${desc.replace(/\|/g, '\\|')} | ${SEVERITY_LABEL[severity] ?? severity} | ${fix ? 'Yes' : 'No'} |`,
    )
  }

  for (const mod of modules) {
    const slug = mod.frontmatter.slug ?? mod.frontmatter.id.toLowerCase().replace(/_/g, '-')
    const page = `rules/${slug}.md`
    for (const c of mod.checks) {
      push(c.id, `${page}#${anchor(c.id)}`, c.category ?? mod.frontmatter.category, c.description, c.severity, c.fix)
    }
    // Single-check rules with no check table link to the page itself.
    if (!mod.checks.length) {
      push(mod.frontmatter.id, page, mod.frontmatter.category, mod.description, mod.frontmatter.severity, mod.frontmatter.fix)
    }
  }

  // Legacy generated modules: page-level links only.
  for (const mod of Object.keys(LEGACY_MODULES)) {
    const page = `rules/${legacyPageName(mod)}`
    const catKey = LEGACY_CATEGORY[mod].toLowerCase().replace(/ /g, '-')
    for (const [id, desc] of legacyRows(mod)) {
      push(id, page, catKey, desc, LEGACY_SEVERITIES[mod].toLowerCase(), false)
    }
  }

  // Hand-written / data-driven catalog rows (migrated modules are skipped via
  // the `seen` set once their schema pages exist).
  for (const [id, page, category, severity, desc] of catalogRows()) {
    push(id, `${page}#${anchor(id)}`, category, desc, severity, false)
  }

  return rows.join('\n') + '\n'
}

function renderDataDrivenBody(): string {
  const compat = parseChecks(COMPAT_TOML)
  const si = parseChecks(SI_TOML)

  const byCategory = new Map<string, CheckEntry[]>()
  for (const entry of compat) {
    const cat = entry.category || 'compatibility'
    if (!byCategory.has(cat)) byCategory.set(cat, [])
    byCategory.get(cat)!.push(entry)
  }

  const CATEGORY_LABEL: Record<string, string> = {
    compatibility: 'Compatibility Considerations',
    'forward-compatibility': 'Forward Compatibility',
    'behavior-changes': 'Behavior Changes',
  }

  const parts: string[] = []
  for (const [cat, entries] of byCategory) {
    const label = CATEGORY_LABEL[cat] ?? cat
    parts.push(`### ${label} (${entries.length} checks)\n\n` + dataTable(entries, 'rules/compatibility.md'))
  }
  parts.push(`### Suggested Improvements (${si.length} checks)\n\n` + dataTable(si, 'rules/suggested-improvements.md'))
  return parts.join('\n')
}

function dataTable(entries: CheckEntry[], page?: string): string {
  if (!entries.length) return '_(no checks)_\n'
  const lines = ['| Check ID | Message |', '| -------- | ------- |']
  for (const entry of entries) {
    const msg = (entry.message || '').replace(/\|/g, '\\|')
    const id = `\`${entry.id}\``
    const link = page ? `[\`${entry.id}\`](${page}#${anchor(entry.id ?? '')})` : id
    lines.push(`| ${link} | ${msg} |`)
  }
  return lines.join('\n') + '\n'
}

function renderGeneratedRulesSection(): string {
  const intro =
    'These tables are generated from the TOML data files by the TypeScript\n' +
    'generator in `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`)\n' +
    'to refresh them whenever `data/compatibility.toml` or\n' +
    '`data/suggested_improvements.toml` changes.\n'
  return '## Data-Driven Check IDs\n\n' + intro + '\n' + renderDataDrivenBody()
}

// ---------------------------------------------------------------------------
// Writers
// ---------------------------------------------------------------------------

function writeRulePages(modules: RuleModule[]): void {
  for (const mod of modules) {
    const out = join(RULES_DIR, rulePageName(mod))
    writeFileSync(out, renderRulePage(mod), 'utf8')
    console.log(`wrote docs/rules/${rulePageName(mod)} (${mod.checks.length || 'single'} check${mod.checks.length === 1 ? '' : 's'})`)
  }
  for (const mod of Object.keys(LEGACY_MODULES)) {
    const icon = LEGACY_ICONS[mod] ?? 'lucide/list-checks'
    const content = `---\nicon: ${icon}\n---\n\n` + renderLegacyBody(mod, legacyRows(mod))
    const out = join(RULES_DIR, legacyPageName(mod))
    writeFileSync(out, content, 'utf8')
    console.log(`wrote docs/rules/${legacyPageName(mod)} (legacy, ${legacyRows(mod).length} checks)`)
  }
}

function writeRulesMd(modules: RuleModule[]): boolean {
  const text = readFileSync(RULES_MD, 'utf8')

  // 1. Replace the generated Rule Table (marked by `<!-- GENERATED: Rule Table -->`).
  const tableMarker = '<!-- GENERATED: Rule Table -->'
  const tableStart = text.indexOf(tableMarker)
  const generatedTable =
    tableMarker +
    '\n\n' +
    renderRuleTable(modules) +
    '\n' +
    'These rows are generated from the rule docstrings by `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`) to refresh them.\n'

  let newText: string
  if (tableStart === -1) {
    const after = '## Overview\n'
    const idx = text.indexOf(after)
    const insert = idx === -1 ? 0 : idx + after.length
    newText = text.slice(0, insert) + generatedTable + '\n' + text.slice(insert)
  } else {
    const marker = '## Rule Categories'
    const end = text.indexOf(marker)
    const realEnd = end === -1 ? text.length : end
    newText = text.slice(0, tableStart) + generatedTable + '\n' + text.slice(realEnd)
  }
  newText = replaceDataDrivenSection(newText)
  writeFileSync(RULES_MD, newText, 'utf8')
  console.log('Updated docs/rules.md')
  return true
}

function replaceDataDrivenSection(text: string): string {
  const generated = renderGeneratedRulesSection()
  const marker = '## Rule Categories'
  const idx = text.indexOf(marker)
  if (idx === -1) return text
  const sectionHeading = '## Data-Driven Check IDs\n'
  const oldStart = text.indexOf(sectionHeading)
  if (oldStart !== -1 && oldStart < idx) {
    text = text.slice(0, oldStart) + text.slice(idx)
  }
  const newIdx = text.indexOf(marker)
  return text.slice(0, newIdx) + generated + '\n' + text.slice(newIdx)
}

// ---------------------------------------------------------------------------
// --check: fail when any generated file has drifted from the sources
// ---------------------------------------------------------------------------

function checkDrift(modules: RuleModule[], only?: string): boolean {
  let clean = true

  for (const mod of modules) {
    if (only && rulePageName(mod).replace(/\.md$/, '') !== only) continue
    const expected = renderRulePage(mod)
    const out = join(RULES_DIR, rulePageName(mod))
    if (!existsSync(out) || readFileSync(out, 'utf8') !== expected) {
      console.error(`docs/rules/${rulePageName(mod)} is out of date (run \`bun run docs:gen\`)`)
      clean = false
    }
  }
  for (const mod of Object.keys(LEGACY_MODULES)) {
    if (only && legacyPageName(mod).replace(/\.md$/, '') !== only) continue
    const icon = LEGACY_ICONS[mod] ?? 'lucide/list-checks'
    const expected = `---\nicon: ${icon}\n---\n\n` + renderLegacyBody(mod, legacyRows(mod))
    const out = join(RULES_DIR, legacyPageName(mod))
    if (!existsSync(out) || readFileSync(out, 'utf8') !== expected) {
      console.error(`docs/rules/${legacyPageName(mod)} is out of date (run \`bun run docs:gen\`)`)
      clean = false
    }
  }

  if (only) return clean

  const text = readFileSync(RULES_MD, 'utf8')
  const tableMarker = '<!-- GENERATED: Rule Table -->'
  const tableStart = text.indexOf(tableMarker)
  const expectedTable = tableMarker + '\n\n' + renderRuleTable(modules) + '\n' + 'These rows are generated from the rule docstrings by `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`) to refresh them.\n'
  if (tableStart === -1 || !text.startsWith(expectedTable, tableStart)) {
    console.error('docs/rules.md generated Rule Table is out of date (run `bun run docs:gen`)')
    clean = false
  }

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

  return clean
}

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

const USAGE = `Generate mlt documentation tables and category pages.

CLI:
    bun docs/scripts/gen_rules_docs.ts --print-data     # print data-driven tables
    bun docs/scripts/gen_rules_docs.ts --write          # write rule pages + rules.md (default)
    bun docs/scripts/gen_rules_docs.ts --check          # exit 1 if any generated file is stale
    bun docs/scripts/gen_rules_docs.ts --check-module <slug>  # check a single rule page only`

function main(): number {
  const args = process.argv.slice(2)
  const has = (flag: string) => args.includes(flag)

  if (has('--print-data')) {
    process.stdout.write(renderDataDrivenBody())
    return 0
  }
  if (has('--check-module')) {
    const idx = args.indexOf('--check-module')
    const slug = args[idx + 1]
    if (!slug) {
      console.error('usage: --check-module <slug>')
      return 1
    }
    return checkDrift(discoverModules(), slug) ? 0 : 1
  }
  if (has('--check')) {
    const modules = discoverModules()
    return checkDrift(modules) ? 0 : 1
  }
  if (has('--help')) {
    console.log(USAGE)
    return 0
  }
  const modules = discoverModules()
  writeRulePages(modules)
  if (!writeRulesMd(modules)) return 1
  return 0
}

if (import.meta.main) {
  process.exit(main())
}
