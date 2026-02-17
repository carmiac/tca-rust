use anyhow::Result;
use clap::{Parser, Subcommand};

mod export;
mod shared;
mod validate;

#[derive(Parser)]
#[command(name = "tca")]
#[command(about = "Terminal Colors Architecture - Theme management toolkit", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a TCA theme file
    Validate {
        /// Path to the theme YAML file or theme name from shared directory
        theme: String,
        /// Path to the schema file
        schema_path: String,
    },
    /// Export a theme to various formats
    Export {
        /// Path to the theme YAML file or theme name from shared directory
        theme: String,
        /// Output format (kitty, alacritty, base16, vim, helix, starship, vscode, iterm2, tmux)
        #[arg(value_name = "FORMAT")]
        format: String,
        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List available themes from user local shared directory
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { theme, schema_path } => {
            validate::run(&theme, &schema_path)?;
        }
        Commands::Export {
            theme,
            format,
            output,
        } => {
            export::run(&theme, &format, output.as_deref())?;
        }
        Commands::List => match tca_loader::get_themes_dir() {
            Ok(dir) => {
                println!("Themes directory: {}", dir.display());
                println!();
                match tca_loader::list_theme_names() {
                    Ok(themes) => {
                        if themes.is_empty() {
                            println!("No themes found.");
                            println!("Add .yaml theme files to: {}", dir.display());
                        } else {
                            println!("Available themes ({}):", themes.len());
                            for theme in themes {
                                println!("  - {}", theme);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing themes: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error getting themes directory: {}", e);
            }
        },
    }

    Ok(())
}
