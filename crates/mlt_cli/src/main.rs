use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use clap::Parser;

use mlt_core::{Config, Diagnostic, Linter, RuleRegistry, Severity};

/// mlt — An ultra-fast, extensible linter for MATLAB.
#[derive(Parser, Debug)]
#[command(name = "mlt", version, about)]
struct Cli {
    /// MATLAB files to lint.
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Apply auto-fixes where available.
    #[arg(long)]
    fix: bool,

    /// Path to config file (default: .mlt.toml in current directory).
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("error: {err:#}");
        process::exit(2);
    }
}

fn run(cli: Cli) -> Result<()> {
    // Load configuration from .mlt.toml (or --config path).
    let config = load_config(cli.config.as_deref())?;

    // Build rule registry from configuration (only enabled rules).
    let rules = mlt_rules::active_rules(&config);
    let registry = RuleRegistry::new(rules, &config);
    let mut linter = Linter::new(registry);

    let mut total_diagnostics = 0;
    let mut files_with_issues = 0;

    for file_path in &cli.files {
        let source = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // NOFIL — file not found.
                let diag = Diagnostic {
                    rule_id: "NOFIL",
                    message: format!(
                        "Unable to open file {}. File is not found.",
                        file_path.display()
                    ),
                    severity: Severity::Error,
                    byte_range: 0..1,
                    line: 1,
                    column: 1,
                    fix: None,
                };
                print_diagnostic(file_path, &diag);
                total_diagnostics += 1;
                files_with_issues += 1;
                continue; // skip lint + fix; never write an empty file to a missing path
            }
            Err(_) => {
                // RDERR — non-NotFound I/O error (permission denied, directory, etc.).
                // Catch-all Err(_) is intentional: std::io::ErrorKind has no stable
                // IsADirectory variant.
                let diag = Diagnostic {
                    rule_id: "RDERR",
                    message: format!("Unable to read file {}.", file_path.display()),
                    severity: Severity::Error,
                    byte_range: 0..1,
                    line: 1,
                    column: 1,
                    fix: None,
                };
                print_diagnostic(file_path, &diag);
                total_diagnostics += 1;
                files_with_issues += 1;
                continue;
            }
        };

        let diagnostics = linter
            .lint(&source, file_path)
            .with_context(|| format!("failed to parse {}", file_path.display()))?;

        if diagnostics.is_empty() {
            continue;
        }

        if cli.fix {
            let fixed = apply_fixes(&source, &diagnostics);
            fs::write(file_path, &fixed)
                .with_context(|| format!("failed to write {}", file_path.display()))?;

            let fix_count = diagnostics.iter().filter(|d| d.fix.is_some()).count();
            println!(
                "Fixed {} issue{} in {}",
                fix_count,
                if fix_count == 1 { "" } else { "s" },
                file_path.display()
            );
        } else {
            for diag in &diagnostics {
                print_diagnostic(file_path, diag);
            }
        }

        total_diagnostics += diagnostics.len();
        files_with_issues += 1;
    }

    if !cli.fix && total_diagnostics > 0 {
        println!(
            "\nFound {} issue{} in {} file{}.",
            total_diagnostics,
            if total_diagnostics == 1 { "" } else { "s" },
            files_with_issues,
            if files_with_issues == 1 { "" } else { "s" },
        );
        process::exit(1);
    }

    Ok(())
}

/// Print a diagnostic in the standard `file:line:col [E] ID: message` format.
fn print_diagnostic(file_path: &Path, diag: &Diagnostic) {
    let severity = match diag.severity {
        Severity::Error => "E",
        Severity::Warning => "W",
        Severity::Info => "I",
    };
    println!(
        "{}:{}:{} [{}] {}: {}",
        file_path.display(),
        diag.line,
        diag.column,
        severity,
        diag.rule_id,
        diag.message,
    );
}

/// Discover and load the configuration file.
///
/// Priority:
/// 1. Explicit `--config` path (error if not found).
/// 2. `.mlt.toml` in the current working directory (silent default if missing).
fn load_config(explicit_path: Option<&std::path::Path>) -> Result<Config> {
    let config_path = if let Some(path) = explicit_path {
        if !path.exists() {
            anyhow::bail!("config file not found: {}", path.display());
        }
        Some(path.to_path_buf())
    } else {
        let default_path = PathBuf::from(".mlt.toml");
        if default_path.exists() {
            Some(default_path)
        } else {
            None
        }
    };

    match config_path {
        Some(path) => {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("failed to read config: {}", path.display()))?;
            Config::from_toml(&content)
                .with_context(|| format!("failed to parse config: {}", path.display()))
        }
        None => Ok(Config::default()),
    }
}

/// Apply all available fixes to the source text.
///
/// Fixes are applied in reverse byte-offset order to preserve earlier offsets.
/// Overlapping fixes are detected and skipped to prevent corruption.
fn apply_fixes(source: &str, diagnostics: &[mlt_core::Diagnostic]) -> String {
    // Collect all edits (primary + additional) from diagnostics that have fixes.
    let mut edits: Vec<&mlt_core::Fix> = Vec::new();
    for diag in diagnostics {
        if let Some(ref fix) = diag.fix {
            edits.push(fix);
            for additional in &fix.additional_edits {
                edits.push(additional);
            }
        }
    }

    // Sort edits by byte_range start in reverse order so applying them
    // back-to-front doesn't invalidate earlier offsets.
    edits.sort_by_key(|e| std::cmp::Reverse(e.byte_range.start));

    let mut result = source.to_string();
    let mut last_edit_start = usize::MAX;

    for fix in edits {
        // Skip overlapping edits: if this fix's range overlaps with the
        // previously applied fix, skip it to prevent corruption.
        if fix.byte_range.end > last_edit_start {
            eprintln!(
                "mlt: warning: skipping overlapping fix at bytes {}..{} (conflicts with edit at {})",
                fix.byte_range.start, fix.byte_range.end, last_edit_start
            );
            continue;
        }
        result.replace_range(fix.byte_range.clone(), &fix.replacement);
        last_edit_start = fix.byte_range.start;
    }

    result
}
