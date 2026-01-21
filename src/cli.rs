use clap::Parser;

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

    // Hidden flag for testing i18n
    #[arg(long, hide = true)]
    pub lang: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
