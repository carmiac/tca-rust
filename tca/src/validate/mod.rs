use tca_types::*;
use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;






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

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        anyhow::bail!("Invalid hex color: {}", hex);
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}

fn relative_luminance(hex: &str) -> Result<f64> {
    let (r, g, b) = hex_to_rgb(hex)?;

    let to_linear = |c: u8| -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };

    let r_linear = to_linear(r);
    let g_linear = to_linear(g);
    let b_linear = to_linear(b);

    Ok(0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear)
}

fn contrast_ratio(l1: f64, l2: f64) -> f64 {
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

fn resolve_color(reference: &str, theme: &Theme) -> Result<String> {
    if reference.starts_with("palette.") {
        let parts: Vec<&str> = reference.split('.').collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid palette reference: {}", reference);
        }

        let ramp_name = parts[1];
        let tone = parts[2];

        if ramp_name == "neutral" {
            theme
                .palette
                .neutral
                .0
                .get(tone)
                .cloned()
                .context(format!("Color not found: {}", reference))
        } else {
            theme
                .palette
                .other
                .get(ramp_name)
                .and_then(|ramp| ramp.0.get(tone))
                .cloned()
                .context(format!("Color not found: {}", reference))
        }
    } else if reference.starts_with("semantic.") {
        let semantic = theme.semantic.as_ref().context("No semantic section")?;
        let key = reference.strip_prefix("semantic.").unwrap();
        let color_ref = semantic
            .get(key)
            .context(format!("Semantic color not found: {}", key))?;
        resolve_color(color_ref, theme)
    } else {
        Ok(reference.to_string())
    }
}

fn validate_schema(theme_content: &str, schema_path: &PathBuf) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    let schema_content = fs::read_to_string(schema_path).context("Failed to read schema file")?;
    let schema: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse schema JSON")?;

    let theme_value: serde_json::Value =
        serde_yaml::from_str(theme_content).context("Failed to parse theme as JSON value")?;

    let compiled = jsonschema::JSONSchema::compile(&schema)
        .map_err(|e| anyhow::anyhow!("Failed to compile schema: {}", e))?;

    if let Err(errors) = compiled.validate(&theme_value) {
        for error in errors {
            result.add_error(format!("Schema validation: {}", error));
        }
    }

    Ok(result)
}

fn validate_contrast(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Validate neutral ramp
    let neutral = &theme.palette.neutral.0;

    // Check that neutral has 5 tones
    let required_tones = ["1", "2", "3", "4", "5"];
    for tone in &required_tones {
        if !neutral.contains_key(*tone) {
            result.add_error(format!("Neutral ramp missing tone: {}", tone));
        }
    }

    if neutral.len() >= 5 {
        let luminances: Result<Vec<f64>> = required_tones
            .iter()
            .map(|t| neutral.get(*t).unwrap())
            .map(|hex| relative_luminance(hex))
            .collect();

        let luminances = luminances?;

        // Check span: L5 - L1 >= 0.40
        let span = luminances[4] - luminances[0];
        if span < 0.40 {
            result.add_error(format!(
                "Neutral ramp span too small: {:.3} (must be >= 0.40)",
                span
            ));
        }

        // Check adjacent steps: Ln - Ln-1 >= 0.01
        for i in 1..luminances.len() {
            let step = luminances[i] - luminances[i - 1];
            if step < 0.01 {
                result.add_warning(format!(
                    "Neutral ramp step {}->{} too small: {:.3} (recommended >= 0.01)",
                    i,
                    i + 1,
                    step
                ));
            }
        }
    }

    // ANSI white/black contrast
    let black_hex = resolve_color(&theme.ansi.black, theme)?;
    let white_hex = resolve_color(&theme.ansi.white, theme)?;
    let black_lum = relative_luminance(&black_hex)?;
    let white_lum = relative_luminance(&white_hex)?;
    let ansi_contrast = contrast_ratio(black_lum, white_lum);

    if ansi_contrast < 2.5 {
        result.add_error(format!(
            "ANSI black/white contrast too low: {:.2} (must be >= 2.5)",
            ansi_contrast
        ));
    } else if ansi_contrast < 3.5 {
        result.add_warning(format!(
            "ANSI black/white contrast low: {:.2} (recommended >= 3.5)",
            ansi_contrast
        ));
    }

    // Semantic colors vs ui.bg.primary
    if let (Some(semantic), Some(ui)) = (&theme.semantic, &theme.ui)
        && let Some(bg_primary_ref) = ui.get("bg.primary") {
        let bg_primary = resolve_color(bg_primary_ref, theme)?;
        let bg_lum = relative_luminance(&bg_primary)?;

        for (name, color_ref) in semantic {
            let color = resolve_color(color_ref, theme)?;
            let color_lum = relative_luminance(&color)?;
            let contrast = contrast_ratio(bg_lum, color_lum);

            if contrast < 2.0 {
                result.add_error(format!(
                    "Semantic '{}' vs bg.primary contrast too low: {:.2} (must be >= 2.0)",
                    name, contrast
                ));
            } else if contrast < 2.5 {
                result.add_warning(format!(
                    "Semantic '{}' vs bg.primary contrast low: {:.2} (recommended >= 2.5)",
                    name, contrast
                ));
            }
        }
    }

    // ui.fg.primary vs ui.bg.primary
    if let Some(ui) = &theme.ui
        && let (Some(fg_ref), Some(bg_ref)) = (ui.get("fg.primary"), ui.get("bg.primary")) {
        let fg = resolve_color(fg_ref, theme)?;
        let bg = resolve_color(bg_ref, theme)?;
        let fg_lum = relative_luminance(&fg)?;
        let bg_lum = relative_luminance(&bg)?;
        let contrast = contrast_ratio(fg_lum, bg_lum);

        if contrast < 3.0 {
            result.add_error(format!(
                "UI fg.primary/bg.primary contrast too low: {:.2} (must be >= 3.0)",
                contrast
            ));
        } else if contrast < 3.5 {
            result.add_warning(format!(
                "UI fg.primary/bg.primary contrast low: {:.2} (recommended >= 3.5)",
                contrast
            ));
        }
    }

    Ok(result)
}

pub fn run(file_path: &str) -> Result<()> {
    let content = tca_loader::load_theme_file(file_path)?;

    let theme: Theme = serde_yaml::from_str(&content).context("Failed to parse theme file")?;

    let mut all_issues = ValidationResult::new();

    // Schema validation
    let schema_path = PathBuf::from("tca.schema.pragmatic.json");
    match validate_schema(&content, &schema_path) {
        Ok(result) => {
            all_issues.issues.extend(result.issues);
        }
        Err(e) => {
            all_issues.add_warning(format!("Schema validation skipped: {}", e));
        }
    }

    // Contrast validation
    let result = validate_contrast(&theme)?;
    all_issues.issues.extend(result.issues);

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
