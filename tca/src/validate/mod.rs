use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use tca_types::*;

#[derive(Debug)]
enum ValidationIssue {
    Error(String),
    Warning(String),
}

struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    fn new() -> Self {
        Self { issues: Vec::new() }
    }

    fn add_error(&mut self, message: String) {
        self.issues.push(ValidationIssue::Error(message));
    }

    fn add_warning(&mut self, message: String) {
        self.issues.push(ValidationIssue::Warning(message));
    }

    fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|i| matches!(i, ValidationIssue::Error(_)))
    }

    fn print(&self) {
        for issue in &self.issues {
            match issue {
                ValidationIssue::Error(msg) => {
                    println!("{} {}", "✗".red().bold(), msg.red());
                }
                ValidationIssue::Warning(msg) => {
                    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
                }
            }
        }
    }
}

fn relative_luminance(hex: &str) -> Result<f64> {
    let (r, g, b) = hex_to_rgb(hex).unwrap_or_else(|_| panic!("Can't parse hex value {}", hex));

    let to_linear = |c: u8| -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };

    Ok(0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b))
}

fn contrast_ratio(l1: f64, l2: f64) -> f64 {
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Check a single contrast pair and emit warning/error based on the spec thresholds.
fn check_contrast(
    result: &mut ValidationResult,
    label: &str,
    fg_hex: &str,
    bg_hex: &str,
    warn_below: f64,
    error_below: f64,
) -> Result<()> {
    let fg_lum = relative_luminance(fg_hex)?;
    let bg_lum = relative_luminance(bg_hex)?;
    let ratio = contrast_ratio(fg_lum, bg_lum);

    if ratio < error_below {
        result.add_error(format!(
            "{} contrast too low: {:.2} (error threshold < {:.1})",
            label, ratio, error_below
        ));
    } else if ratio < warn_below {
        result.add_warning(format!(
            "{} contrast low: {:.2} (recommended >= {:.1})",
            label, ratio, warn_below
        ));
    }
    Ok(())
}

fn validate_schema(theme_content: &str, schema_path: &PathBuf) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    let schema_content = fs::read_to_string(schema_path).context("Failed to read schema file")?;
    let schema: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse schema JSON")?;

    // Convert TOML → JSON for schema validation
    let toml_value: toml::Value =
        toml::from_str(theme_content).context("Failed to parse theme as TOML")?;
    let theme_value: serde_json::Value =
        serde_json::to_value(&toml_value).context("Failed to convert TOML to JSON value")?;

    let compiled = jsonschema::JSONSchema::compile(&schema)
        .map_err(|e| anyhow::anyhow!("Failed to compile schema: {}", e))?;

    if let Err(errors) = compiled.validate(&theme_value) {
        for error in errors {
            result.add_error(format!("Schema validation: {}", error));
        }
    }

    Ok(result)
}

fn validate_ansi_hex(theme: &Theme) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Per spec: ANSI values must be direct hex — no references allowed.
    let check = |name: &str, value: &str| -> Option<ValidationIssue> {
        if !value.starts_with('#') {
            Some(ValidationIssue::Error(format!(
                "ANSI {} must be a direct hex color, got '{}'",
                name, value
            )))
        } else if value.len() != 7 {
            Some(ValidationIssue::Error(format!(
                "ANSI {} invalid hex '{}' (must be #RRGGBB)",
                name, value
            )))
        } else {
            None
        }
    };

    let ansi_fields = [
        ("black", &theme.ansi.black),
        ("red", &theme.ansi.red),
        ("green", &theme.ansi.green),
        ("yellow", &theme.ansi.yellow),
        ("blue", &theme.ansi.blue),
        ("magenta", &theme.ansi.magenta),
        ("cyan", &theme.ansi.cyan),
        ("white", &theme.ansi.white),
        ("bright_black", &theme.ansi.bright_black),
        ("bright_red", &theme.ansi.bright_red),
        ("bright_green", &theme.ansi.bright_green),
        ("bright_yellow", &theme.ansi.bright_yellow),
        ("bright_blue", &theme.ansi.bright_blue),
        ("bright_magenta", &theme.ansi.bright_magenta),
        ("bright_cyan", &theme.ansi.bright_cyan),
        ("bright_white", &theme.ansi.bright_white),
    ];

    for (name, value) in ansi_fields {
        if let Some(issue) = check(name, value) {
            result.issues.push(issue);
        }
    }

    result
}

fn validate_references(theme: &Theme) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Helper: check that a reference can be resolved
    let check_ref = |label: &str, value: &str| -> Option<ValidationIssue> {
        if value.starts_with('#') {
            return None; // direct hex always valid
        }
        if theme.resolve(value).is_none() {
            Some(ValidationIssue::Error(format!(
                "Unresolvable reference in {}: '{}'",
                label, value
            )))
        } else {
            None
        }
    };

    // palette
    if let Some(Palette(palette)) = &theme.palette {
        for (ramp_name, ramp) in palette.iter() {
            for (i, color) in ramp.iter().enumerate() {
                check_ref(format!("palette.{}.{}", ramp_name, i).as_str(), color);
            }
        }
    }
    // base16
    if let Some(Base16(base16)) = &theme.base16 {
        for (key, value) in base16 {
            check_ref(format!("base16.{}", key).as_str(), value);
        }
    }

    // semantic
    macro_rules! check {
        ($label:expr, $val:expr) => {
            if let Some(issue) = check_ref($label, $val) {
                result.issues.push(issue);
            }
        };
    }

    check!("semantic.error", &theme.semantic.error);
    check!("semantic.warning", &theme.semantic.warning);
    check!("semantic.info", &theme.semantic.info);
    check!("semantic.success", &theme.semantic.success);
    check!("semantic.highlight", &theme.semantic.highlight);
    check!("semantic.link", &theme.semantic.link);

    // ui
    check!("ui.bg.primary", &theme.ui.bg.primary);
    check!("ui.bg.secondary", &theme.ui.bg.secondary);
    check!("ui.fg.primary", &theme.ui.fg.primary);
    check!("ui.fg.secondary", &theme.ui.fg.secondary);
    check!("ui.fg.muted", &theme.ui.fg.muted);
    check!("ui.border.primary", &theme.ui.border.primary);
    check!("ui.border.muted", &theme.ui.border.muted);
    check!("ui.cursor.primary", &theme.ui.cursor.primary);
    check!("ui.cursor.muted", &theme.ui.cursor.muted);
    check!("ui.selection.bg", &theme.ui.selection.bg);
    check!("ui.selection.fg", &theme.ui.selection.fg);

    result
}

fn validate_contrast(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    let resolve = |r: &str| theme.resolve(r).unwrap_or_default();

    let bg_primary = resolve(&theme.ui.bg.primary);
    let bg_secondary = resolve(&theme.ui.bg.secondary);

    if bg_primary.is_empty() || bg_secondary.is_empty() {
        return Ok(result);
    }

    // fg.primary / bg.* — recommended >4.5, warn <3.5, error <3.0
    let fg_primary = resolve(&theme.ui.fg.primary);
    if !fg_primary.is_empty() {
        check_contrast(
            &mut result,
            "ui.fg.primary / bg.primary",
            &fg_primary,
            &bg_primary,
            3.5,
            3.0,
        )?;
        check_contrast(
            &mut result,
            "ui.fg.primary / bg.secondary",
            &fg_primary,
            &bg_secondary,
            3.5,
            3.0,
        )?;
    }

    // fg.muted / bg.* — recommended >3.0, warn <2.5, error <2.0
    let fg_muted = resolve(&theme.ui.fg.muted);
    if !fg_muted.is_empty() {
        check_contrast(
            &mut result,
            "ui.fg.muted / bg.primary",
            &fg_muted,
            &bg_primary,
            2.5,
            2.0,
        )?;
    }

    // cursor.primary / bg.* — recommended >4.5, warn <3.5, error <3.0
    let cursor_primary = resolve(&theme.ui.cursor.primary);
    if !cursor_primary.is_empty() {
        check_contrast(
            &mut result,
            "ui.cursor.primary / bg.primary",
            &cursor_primary,
            &bg_primary,
            3.5,
            3.0,
        )?;
    }

    // cursor.muted / bg.* — recommended >3.0, warn <2.5, error <2.0
    let cursor_muted = resolve(&theme.ui.cursor.muted);
    if !cursor_muted.is_empty() {
        check_contrast(
            &mut result,
            "ui.cursor.muted / bg.primary",
            &cursor_muted,
            &bg_primary,
            2.5,
            2.0,
        )?;
    }

    // border.* / bg.* — recommended >3.0, warn <2.5, error <2.0
    let border_primary = resolve(&theme.ui.border.primary);
    if !border_primary.is_empty() {
        check_contrast(
            &mut result,
            "ui.border.primary / bg.primary",
            &border_primary,
            &bg_primary,
            2.5,
            2.0,
        )?;
    }

    // selection.fg / selection.bg — recommended >3.0, warn <2.5, error <2.0
    let sel_bg = resolve(&theme.ui.selection.bg);
    let sel_fg = resolve(&theme.ui.selection.fg);
    if !sel_bg.is_empty() && !sel_fg.is_empty() {
        check_contrast(
            &mut result,
            "ui.selection.fg / selection.bg",
            &sel_fg,
            &sel_bg,
            2.5,
            2.0,
        )?;
    }

    // semantic.* / bg.primary — recommended >4.5, warn <2.5, error <2.0
    for (name, color_ref) in [
        ("error", &theme.semantic.error),
        ("warning", &theme.semantic.warning),
        ("info", &theme.semantic.info),
        ("success", &theme.semantic.success),
        ("highlight", &theme.semantic.highlight),
        ("link", &theme.semantic.link),
    ] {
        let color = resolve(color_ref);
        if !color.is_empty() {
            check_contrast(
                &mut result,
                &format!("semantic.{} / bg.primary", name),
                &color,
                &bg_primary,
                2.5,
                2.0,
            )?;
        }
    }

    Ok(result)
}

pub fn run(file_path: &str, schema_path: Option<String>) -> Result<()> {
    let content = tca_loader::load_theme_file(file_path)?;

    let theme: Theme = toml::from_str(&content).context("Failed to parse theme file as TOML")?;

    let mut all_issues = ValidationResult::new();

    // Schema validation (optional — skip gracefully if schema unavailable)
    if let Some(schema_path) = schema_path {
        let schema_path_buf = PathBuf::from(schema_path);
        match validate_schema(&content, &schema_path_buf) {
            Ok(r) => all_issues.issues.extend(r.issues),
            Err(e) => all_issues.add_warning(format!("Could not validate from schema {}", e)),
        }
    } else {
        all_issues.add_warning("Schema validation skipped.".into());
    }

    // ANSI values must be hex
    all_issues.issues.extend(validate_ansi_hex(&theme).issues);

    // Reference resolution
    all_issues.issues.extend(validate_references(&theme).issues);

    // Contrast (recommended)
    let contrast_result = validate_contrast(&theme)?;
    all_issues.issues.extend(contrast_result.issues);

    // Print results
    if all_issues.issues.is_empty() {
        println!("{} Theme validation passed!", "✓".green().bold());
        Ok(())
    } else {
        all_issues.print();
        println!();
        if all_issues.has_errors() {
            println!("{} Validation failed with errors", "✗".red().bold());
            std::process::exit(1);
        } else {
            println!("{} Validation passed with warnings", "⚠".yellow().bold());
            Ok(())
        }
    }
}
