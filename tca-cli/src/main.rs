use anyhow::Result;
use clap::{Parser, Subcommand};

mod add;
mod config;
mod export;
mod list;
mod validate;

#[derive(Parser)]
#[command(name = "tca")]
#[command(about = "Terminal Colors Architecture - Theme management toolkit", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Validate a TCA theme file
    Validate {
        /// Path to the theme TOML file or theme name from shared directory
        theme: String,
        /// Path to the schema file
        schema_path: Option<String>,
    },
    /// Export a theme to various formats
    Export {
        /// Path to the theme TOML file or theme name from shared directory
        theme: String,
        /// Output format (kitty, alacritty, base16, vim, helix, starship, vscode, iterm2, tmux)
        #[arg(value_name = "FORMAT")]
        format: String,
        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List all available themes.
    List,
    /// Show/set user configuration values. By default will show entire config.
    Config {
        #[command(subcommand)]
        cmd: Option<ConfigCommand>,
    },
    /// Add a theme to the common directory from a given file, directory, or download by name.
    ///
    /// Supports somewhat fuzzy names when downloading themes, so "Tokyo Nights", "tokyo-nights", "tokyoNights", etc.
    /// would all get the same theme.
    /// 'tca add /path/to/theme.toml'
    /// 'tca add /path/to/theme/dir/'
    /// 'tca add tokyo-nights'
    /// 'tca add "Tokyo Nights"
    Add {
        theme: Option<Vec<String>>,
        /// Download all themes from the repository.
        #[arg(long)]
        all: bool,
        /// Remote repo URL.
        #[arg(short, long, default_value = "git@github.com:carmiac/tca-themes.git")]
        repo_url: String,
        /// Theme directory in repo.
        #[arg(short, long, default_value = "themes/")]
        dir_path: String,
        /// Repo branch.
        #[arg(short, long, default_value = "main")]
        branch: String,
    },
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigCommand {
    /// Show the current config. Default if no options.
    Show,
    /// Set config value.
    Set {
        /// Key should be one of default, default_dark, default_light.
        key: String,
        /// The theme you are setting.
        theme: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { theme, schema_path } => {
            validate::run(&theme, schema_path)?;
        }
        Commands::Export {
            theme,
            format,
            output,
        } => {
            export::run(&theme, &format, output.as_deref())?;
        }
        Commands::List => {
            list::run()?;
        }
        Commands::Config { cmd } => config::run(&cmd)?,
        Commands::Add {
            theme,
            all,
            repo_url,
            dir_path,
            branch,
        } => add::run(&theme, all, repo_url, dir_path, branch)?,
    }

    Ok(())
}
