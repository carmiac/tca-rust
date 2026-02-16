use tca_types::*;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

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

fn validate_palette_structure(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Check neutral ramp
    let len = theme.palette.neutral.0.len();
    if len != 5 {
        result.add_error(format!("Neutral ramp missing entries. Must be 5 long, only found {}",len));
    }
    for (key,_) in theme.palette.neutral.0.iter() {
        if !matches!(key.as_str(), "1" | "2" | "3" | "4" | "5") {
            result.add_error(format!(
                "Invalid ramp key in neutral: '{}' (must be 1-5)",
                key
            ));
        }
    }

    // Check other ramps
    for (ramp_name, ramp) in &theme.palette.other {
        for key in ramp.0.keys() {
            if !matches!(key.as_str(), "1" | "2" | "3" | "4" | "5") {
                result.add_error(format!(
                    "Invalid ramp key in {}: '{}' (must be 1-5)",
                    ramp_name, key
                ));
            }
        }
    }

    // Validate all palette values are hex colors
    for (key, value) in &theme.palette.neutral.0 {
        if !value.starts_with('#') || value.len() != 7 {
            result.add_error(format!(
                "Invalid hex color in neutral.{}: '{}' (must be #RRGGBB)",
                key, value
            ));
        }
    }
    for (ramp_name, ramp) in &theme.palette.other {
        for (key, value) in &ramp.0 {
            if !value.starts_with('#') || value.len() != 7 {
                result.add_error(format!(
                    "Invalid hex color in {}.{}: '{}' (must be #RRGGBB)",
                    ramp_name, key, value
                ));
            }
        }
    }

    Ok(result)
}

fn validate_no_direct_hex(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Helper to check if a value is a direct hex color
    let is_hex = |s: &str| s.starts_with('#');

    // Check ANSI colors
    if is_hex(&theme.ansi.black) { result.add_error("ANSI black uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.red) { result.add_error("ANSI red uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.green) { result.add_error("ANSI green uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.yellow) { result.add_error("ANSI yellow uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.blue) { result.add_error("ANSI blue uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.magenta) { result.add_error("ANSI magenta uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.cyan) { result.add_error("ANSI cyan uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.white) { result.add_error("ANSI white uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_black) { result.add_error("ANSI brightBlack uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_red) { result.add_error("ANSI brightRed uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_green) { result.add_error("ANSI brightGreen uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_yellow) { result.add_error("ANSI brightYellow uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_blue) { result.add_error("ANSI brightBlue uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_magenta) { result.add_error("ANSI brightMagenta uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_cyan) { result.add_error("ANSI brightCyan uses direct hex (must use palette reference)".to_string()); }
    if is_hex(&theme.ansi.bright_white) { result.add_error("ANSI brightWhite uses direct hex (must use palette reference)".to_string()); }

    // Check semantic colors
    if let Some(semantic) = &theme.semantic {
        for (name, value) in semantic {
            if is_hex(value) {
                result.add_error(format!(
                    "Semantic {} uses direct hex (must use palette reference)",
                    name
                ));
            }
        }
    }

    // Check UI colors
    if let Some(ui) = &theme.ui {
        for (name, value) in ui {
            if is_hex(value) {
                result.add_error(format!(
                    "UI {} uses direct hex (must use palette reference)",
                    name
                ));
            }
        }
    }

    // Check base16 colors
    if let Some(base16) = &theme.base16 {
        for (name, value) in &base16.0 {
            if is_hex(value) {
                result.add_error(format!(
                    "Base16 {} uses direct hex (must use palette reference)",
                    name
                ));
            }
        }
    }

    Ok(result)
}

fn validate_contrast(theme: &Theme) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Validate neutral ramp (for contrast checking only - existence checked elsewhere)
    let neutral = &theme.palette.neutral.0;

    // Only check contrast if we have enough tones for meaningful testing
    let required_tones = ["1", "2", "3", "4", "5"];
    let has_all_tones = required_tones.iter().all(|t| neutral.contains_key(*t));

    if has_all_tones {
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
    let black_hex = theme.resolve(&theme.ansi.black).unwrap_or_default();
    let white_hex = theme.resolve(&theme.ansi.white).unwrap_or_default();
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
    if let (Some(semantic), Some(ui)) = (&theme.semantic, &theme.ui) {
        if let Some(bg_primary_ref) = ui.get("bg.primary") {
            let bg_primary = theme.resolve(bg_primary_ref).unwrap_or_default();
            let bg_lum = relative_luminance(&bg_primary)?;

            for (name, color_ref) in semantic {
                let color = theme.resolve(color_ref).unwrap_or_default();
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
    }

    // ui.fg.primary vs ui.bg.primary
    if let Some(ui) = &theme.ui {
        if let Some(fg_ref) = ui.get("fg.primary") {
            if let Some(bg_ref) = ui.get("bg.primary") {
                let fg = theme.resolve(fg_ref).unwrap_or_default();
                let bg = theme.resolve(bg_ref).unwrap_or_default();
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

    // Palette structure validation (strict)
    let result = validate_palette_structure(&theme)?;
    all_issues.issues.extend(result.issues);

    // Direct hex validation (strict)
    let result = validate_no_direct_hex(&theme)?;
    all_issues.issues.extend(result.issues);

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
