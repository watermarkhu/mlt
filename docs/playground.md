---
icon: lucide/play-circle
hide:
  - toc
---

# Playground

Try mlt directly in your browser — no installation needed. Powered by [mlt-wasm](https://github.com/watermarkhu/mlt), the same linting engine compiled to WebAssembly.

<div id="mlt-playground">

<div id="pg-status" class="pg-status pg-loading">Loading mlt-wasm…</div>

<div class="pg-toolbar" id="pg-toolbar" style="display:none">
  <span id="pg-version" class="pg-version"></span>
  <div class="pg-toolbar-right">
    <select id="pg-example" class="pg-select">
      <option value="">Load an example…</option>
      <option value="common">Common issues</option>
      <option value="prealloc">Loop preallocation</option>
      <option value="class">Class design</option>
      <option value="clean">Clean document</option>
    </select>
    <button id="pg-fix-btn" class="pg-btn pg-btn-primary" disabled>Fix all</button>
    <button id="pg-clear-btn" class="pg-btn">Clear</button>
  </div>
</div>

<div class="pg-panels" id="pg-panels" style="display:none">
  <div class="pg-panel">
    <div class="pg-panel-header">
      <span>MATLAB input</span>
      <span id="pg-char-count" class="pg-meta"></span>
    </div>
    <textarea id="pg-input" class="pg-editor" spellcheck="false" placeholder="Type or paste MATLAB code here…"></textarea>
  </div>
  <div class="pg-panel">
    <div class="pg-panel-header">
      <span>Warnings</span>
      <span id="pg-warning-count" class="pg-meta"></span>
    </div>
    <div id="pg-warnings" class="pg-results"></div>
  </div>
</div>

</div>

<style>
#mlt-playground {
  margin-top: 1rem;
  font-family: var(--md-text-font-family, inherit);
}

.pg-status {
  padding: 1rem;
  border-radius: 4px;
  font-size: 0.85rem;
  text-align: center;
}
.pg-loading {
  background: var(--md-code-bg-color, #f5f5f5);
  color: var(--md-default-fg-color--light, #666);
}
.pg-error {
  background: #fff0f0;
  color: #c0392b;
  border: 1px solid #f5c6cb;
}

.pg-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
  flex-wrap: wrap;
}
.pg-toolbar-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
}
.pg-version {
  font-size: 0.75rem;
  color: var(--md-default-fg-color--light, #888);
  font-family: var(--md-code-font-family, monospace);
}

.pg-select {
  font-size: 0.8rem;
  padding: 0.3rem 0.5rem;
  border: 1px solid var(--md-default-fg-color--lighter, #ddd);
  border-radius: 4px;
  background: var(--md-default-bg-color, #fff);
  color: var(--md-default-fg-color, #333);
  cursor: pointer;
}

.pg-btn {
  font-size: 0.8rem;
  padding: 0.35rem 0.85rem;
  border: 1px solid var(--md-default-fg-color--lighter, #ddd);
  border-radius: 4px;
  background: var(--md-default-bg-color, #fff);
  color: var(--md-default-fg-color, #333);
  cursor: pointer;
  transition: opacity 0.15s;
}
.pg-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.pg-btn-primary {
  background: var(--md-primary-fg-color, #1565c0);
  color: var(--md-primary-bg-color, #fff);
  border-color: transparent;
}
.pg-btn-primary:not(:disabled):hover {
  opacity: 0.85;
}

.pg-panels {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}
@media (max-width: 768px) {
  .pg-panels { grid-template-columns: 1fr; }
}

.pg-panel {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--md-default-fg-color--lighter, #ddd);
  border-radius: 6px;
  overflow: hidden;
  background: var(--md-default-bg-color, #fff);
  min-height: 460px;
}

.pg-panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0.75rem;
  background: var(--md-code-bg-color, #f5f5f5);
  border-bottom: 1px solid var(--md-default-fg-color--lighter, #ddd);
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--md-default-fg-color--light, #555);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.pg-meta {
  font-weight: 400;
  font-size: 0.75rem;
  color: var(--md-default-fg-color--light, #888);
  text-transform: none;
  letter-spacing: 0;
}

.pg-editor {
  flex: 1;
  min-height: 360px;
  padding: 0.75rem;
  font-family: var(--md-code-font-family, 'Roboto Mono', monospace);
  font-size: 0.82rem;
  line-height: 1.6;
  border: none;
  outline: none;
  resize: vertical;
  background: var(--md-default-bg-color, #fff);
  color: var(--md-default-fg-color, #333);
  tab-size: 2;
}

.pg-results {
  flex: 1;
  min-height: 360px;
  overflow-y: auto;
  padding: 0.5rem;
}

.pg-warning {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  padding: 0.5rem 0.6rem;
  border-radius: 4px;
  margin-bottom: 0.4rem;
  border-left: 3px solid var(--md-accent-fg-color, #f57f17);
  background: var(--md-code-bg-color, #fafafa);
  font-size: 0.82rem;
}
.pg-warning:last-child { margin-bottom: 0; }

.pg-warning-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.pg-rule-badge {
  font-family: var(--md-code-font-family, monospace);
  font-size: 0.72rem;
  font-weight: 700;
  padding: 0.1rem 0.4rem;
  border-radius: 3px;
  background: var(--md-primary-fg-color, #1565c0);
  color: var(--md-primary-bg-color, #fff);
  white-space: nowrap;
}

.pg-severity {
  font-size: 0.68rem;
  padding: 0.1rem 0.35rem;
  border-radius: 3px;
  background: #e8f5e9;
  color: #2e7d32;
}
.pg-severity.Error {
  background: #fdecea;
  color: #c0392b;
}
.pg-severity.Info {
  background: #e3f2fd;
  color: #1565c0;
}

.pg-location {
  font-family: var(--md-code-font-family, monospace);
  font-size: 0.72rem;
  color: var(--md-default-fg-color--light, #888);
}

.pg-fix-badge {
  font-size: 0.68rem;
  padding: 0.1rem 0.35rem;
  border-radius: 3px;
  background: #fff3e0;
  color: #e65100;
  margin-left: auto;
}

.pg-warning-message {
  color: var(--md-default-fg-color, #333);
  line-height: 1.4;
}

.pg-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 200px;
  color: var(--md-default-fg-color--light, #888);
  font-size: 0.85rem;
  text-align: center;
  padding: 1rem;
}

.pg-clean {
  color: #2e7d32;
  font-weight: 500;
}
</style>

<script type="module">
// Load the locally-built wasm when present (after `mise run serve-local-wasm`),
// otherwise fall back to the published CDN build.
const CDN_URL = 'https://cdn.jsdelivr.net/npm/mlt-wasm/mlt_lib.js';
const LOCAL_URL = '/assets/mlt-wasm/mlt_lib.js';

const EXAMPLES = {
  common: `x = 1
for i = 1:10
    results(i) = i * 2
end
disp(results)`,
  prealloc: `% Without preallocation, the array grows inside the loop.
results = [];
for i = 1:1000
    results = [results, i^2];
end

% Preallocate instead: AGROW won't fire.
results = zeros(1, 1000);
for i = 1:1000
    results(i) = i^2;
end`,
  class: `classdef MyClass < handle
    properties
        Value
    end
    methods
        function set.Value(obj, val)
            obj.Value = val;
        end
        function obj = MyClass()
            obj.Value = 0;
        end
    end
end`,
  clean: `function output = compute(input)
    % A well-formed function: semicolons, preallocation, and clear names.
    arguments
        input double
    end
    output = zeros(1, numel(input));
    for idx = 1:numel(input)
        output(idx) = input(idx) * 2;
    end
end`,
};

const statusEl = document.getElementById('pg-status');
const toolbarEl = document.getElementById('pg-toolbar');
const panelsEl = document.getElementById('pg-panels');
const versionEl = document.getElementById('pg-version');
const inputEl = document.getElementById('pg-input');
const warningsEl = document.getElementById('pg-warnings');
const warningCountEl = document.getElementById('pg-warning-count');
const charCountEl = document.getElementById('pg-char-count');
const fixBtn = document.getElementById('pg-fix-btn');
const clearBtn = document.getElementById('pg-clear-btn');
const exampleSelect = document.getElementById('pg-example');

let linter = null;
let debounceTimer = null;

async function loadWasm() {
  // Try the local build first so the docs site works fully offline.
  try {
    return await import(LOCAL_URL);
  } catch (e) {
    return await import(CDN_URL);
  }
}

async function main() {
  try {
    const mod = await loadWasm();
    await mod.default();

    const version = mod.Linter.get_version();
    linter = new mod.Linter({});

    versionEl.textContent = `mlt v${version}`;
    statusEl.style.display = 'none';
    toolbarEl.style.display = '';
    panelsEl.style.display = '';
    fixBtn.disabled = false;

    inputEl.value = EXAMPLES.common;
    updateCharCount();
    lint();
  } catch (err) {
    statusEl.textContent = `Failed to load mlt-wasm: ${err.message}`;
    statusEl.className = 'pg-status pg-error';
  }
}

function lint() {
  if (!linter) return;
  const content = inputEl.value;
  const warnings = JSON.parse(linter.check(content, null));
  renderWarnings(warnings);
}

function renderWarnings(warnings) {
  warningCountEl.textContent = warnings.length === 0
    ? 'no issues'
    : `${warnings.length} issue${warnings.length === 1 ? '' : 's'}`;

  if (warnings.length === 0) {
    warningsEl.innerHTML = '<div class="pg-empty pg-clean">No issues found</div>';
    return;
  }

  warningsEl.innerHTML = warnings.map(w => {
    const hasFix = w.fix != null;
    return `<div class="pg-warning">
      <div class="pg-warning-header">
        <span class="pg-rule-badge">${escapeHtml(w.rule_id || 'unknown')}</span>
        <span class="pg-severity ${escapeHtml(w.severity)}">${escapeHtml(w.severity)}</span>
        <span class="pg-location">Line ${w.line}:${w.column}</span>
        ${hasFix ? '<span class="pg-fix-badge">auto-fix</span>' : ''}
      </div>
      <div class="pg-warning-message">${escapeHtml(w.message)}</div>
    </div>`;
  }).join('');
}

function updateCharCount() {
  const len = inputEl.value.length;
  charCountEl.textContent = `${len} char${len === 1 ? '' : 's'}`;
}

function escapeHtml(text) {
  const d = document.createElement('div');
  d.textContent = text;
  return d.innerHTML;
}

inputEl.addEventListener('input', () => {
  updateCharCount();
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(lint, 300);
});

fixBtn.addEventListener('click', () => {
  if (!linter) return;
  inputEl.value = linter.fix(inputEl.value, null);
  updateCharCount();
  lint();
});

clearBtn.addEventListener('click', () => {
  inputEl.value = '';
  exampleSelect.value = '';
  updateCharCount();
  lint();
});

exampleSelect.addEventListener('change', () => {
  const key = exampleSelect.value;
  if (key && EXAMPLES[key]) {
    inputEl.value = EXAMPLES[key];
    updateCharCount();
    lint();
  }
});

main();
</script>
