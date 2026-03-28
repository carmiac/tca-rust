use anyhow::Result;
use colored::Colorize;
use tca_types::{hex_to_rgb, Theme};

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
                    eprintln!("{} {}", "✗".red().bold(), msg.red());
                }
                ValidationIssue::Warning(msg) => {
                    eprintln!("{} {}", "⚠".yellow().bold(), msg.yellow());
                }
            }
        }
    }
}

fn relative_luminance(hex: &str) -> Result<f64> {
    let (r, g, b) =
        hex_to_rgb(hex).map_err(|e| anyhow::anyhow!("Can't parse hex value {}: {}", hex, e))?;

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

fn validate_contrast(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    let bg_primary = &theme.ui.bg.primary;
    let bg_secondary = &theme.ui.bg.secondary;

    // fg.primary / bg.* — warn <3.5, error <3.0
    check_contrast(
        &mut result,
        "ui.fg.primary / bg.primary",
        &theme.ui.fg.primary,
        bg_primary,
        3.5,
        3.0,
    )?;
    check_contrast(
        &mut result,
        "ui.fg.primary / bg.secondary",
        &theme.ui.fg.primary,
        bg_secondary,
        3.5,
        3.0,
    )?;

    // fg.secondary / bg.primary — warn <3.5, error <3.0
    check_contrast(
        &mut result,
        "ui.fg.secondary / bg.primary",
        &theme.ui.fg.secondary,
        bg_primary,
        3.5,
        3.0,
    )?;

    // fg.muted / bg.primary — warn <2.5, error <2.0
    check_contrast(
        &mut result,
        "ui.fg.muted / bg.primary",
        &theme.ui.fg.muted,
        bg_primary,
        2.5,
        2.0,
    )?;

    // cursor.primary / bg.primary — warn <3.5, error <3.0
    check_contrast(
        &mut result,
        "ui.cursor.primary / bg.primary",
        &theme.ui.cursor.primary,
        bg_primary,
        3.5,
        3.0,
    )?;

    // cursor.muted / bg.primary — warn <2.5, error <2.0
    check_contrast(
        &mut result,
        "ui.cursor.muted / bg.primary",
        &theme.ui.cursor.muted,
        bg_primary,
        2.5,
        2.0,
    )?;

    // border.primary / bg.primary — warn <2.5, error <2.0
    check_contrast(
        &mut result,
        "ui.border.primary / bg.primary",
        &theme.ui.border.primary,
        bg_primary,
        2.5,
        2.0,
    )?;

    // border.muted / bg.primary — warn <2.5, error <2.0
    check_contrast(
        &mut result,
        "ui.border.muted / bg.primary",
        &theme.ui.border.muted,
        bg_primary,
        2.5,
        2.0,
    )?;

    // selection.fg / selection.bg — warn <2.5, error <2.0
    check_contrast(
        &mut result,
        "ui.selection.fg / selection.bg",
        &theme.ui.selection.fg,
        &theme.ui.selection.bg,
        2.5,
        2.0,
    )?;

    // semantic.* / bg.primary — warn <3.5, error <3.0
    for (name, color) in [
        ("error", theme.semantic.error.as_str()),
        ("warning", theme.semantic.warning.as_str()),
        ("info", theme.semantic.info.as_str()),
        ("success", theme.semantic.success.as_str()),
        ("highlight", theme.semantic.highlight.as_str()),
        ("link", theme.semantic.link.as_str()),
    ] {
        check_contrast(
            &mut result,
            &format!("semantic.{name} / bg.primary"),
            color,
            bg_primary,
            3.5,
            3.0,
        )?;
    }

    Ok(result)
}

pub fn run(file_path: &str) -> Result<()> {
    let content = tca_types::load_theme_file(file_path)?;

    let theme = Theme::from_base24_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse theme as base24 YAML: {}", e))?;

    let mut all_issues = ValidationResult::new();
    let contrast_result = validate_contrast(&theme)?;
    all_issues.issues.extend(contrast_result.issues);

    if all_issues.issues.is_empty() {
        println!("{} Theme validation passed!", "✓".green().bold());
        Ok(())
    } else {
        all_issues.print();
        println!();
        if all_issues.has_errors() {
            println!("{} Validation failed with errors", "✗".red().bold());
            Err(anyhow::anyhow!("Validation failed with errors"))
        } else {
            println!("{} Validation passed with warnings", "⚠".yellow().bold());
            Ok(())
        }
    }
}
