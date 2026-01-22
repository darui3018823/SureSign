use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
    {usage}

{all-args}{after-help}
")]
pub struct Cli {
    /// Common Name (e.g. myserver.local)
    #[arg(long, short = 'c')]
    pub cn: Option<String>,

    /// Subject Alternative Names (IP or DNS)
    #[arg(long, short = 's', value_delimiter = ',', num_args = 0..)]
    pub sans: Option<Vec<String>>,

    /// Validity days
    #[arg(long, short = 'd')]
    pub days: Option<i64>,

    /// Run without interactive prompts (fail if requirements missing, or use defaults)
    #[arg(long)]
    pub non_interactive: bool,

    /// Use default settings for everything missing
    #[arg(long = "default_settings")]
    pub default_settings: bool,

    /// List available commands/presets
    #[arg(long)]
    pub cmdlist: bool,

    /// Full Mode: Ask for Country, State, City, Org, OrgUnit
    #[arg(long)]
    pub full: bool,

    /// All Mode: Ask for KeyType and all other fields
    #[arg(long)]
    pub all: bool,

    // === New V2.0 arguments ===
    /// Output directory for generated certificates
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    /// Output file name (without extension)
    #[arg(long, short = 'n')]
    pub name: Option<String>,

    /// PFX password (leave empty for no password)
    #[arg(long)]
    pub pfx_password: Option<String>,

    /// Skip overwrite confirmation
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Country Name (2 letter code) - for Full/All mode via CLI
    #[arg(long)]
    pub country: Option<String>,

    /// State or Province Name - for Full/All mode via CLI
    #[arg(long)]
    pub state: Option<String>,

    /// Locality/City Name - for Full/All mode via CLI
    #[arg(long)]
    pub city: Option<String>,

    /// Organization Name - for Full/All mode via CLI
    #[arg(long)]
    pub org: Option<String>,

    /// Organizational Unit Name - for Full/All mode via CLI
    #[arg(long)]
    pub org_unit: Option<String>,

    /// Key type: rsa, ecdsa, or ed25519
    #[arg(long, value_parser = ["rsa", "ecdsa", "ed25519"])]
    pub key_type: Option<String>,

    /// Show verbose output
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(long, short = 'q')]
    pub quiet: bool,

    /// Path to config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    // Hidden flag for testing i18n
    #[arg(long, hide = true)]
    pub lang: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
