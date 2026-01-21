use crate::cli::Cli;
use crate::i18n::t;
use crate::cert::CertOptions;
use inquire::{Text, Confirm, CustomType};
use std::process;

pub fn resolve_options(cli: Cli) -> CertOptions {
    if cli.cmdlist {
        println!("{}", t("cmdlist_header"));
        println!("--default_settings: Generate with defaults (CN=localhost, SAN=127.0.0.1, 365 days)");
        println!("--non-interactive: Fail on missing args (or use defaults)");
        process::exit(0);
    }

    let default_cn = "localhost".to_string();
    let default_sans = vec!["127.0.0.1".to_string()];
    let default_days = 365;

    // Direct return if default_settings is requested
    if cli.default_settings {
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
        };
    }

    // Non-interactive Mode handling
    if cli.non_interactive {
        // If CN is missing in non-interactive, should we fail or default?
        // Requirement 3-1: "if missing, error exit or use defaults"
        // Let's use defaults if allowed, but strict non-interactive usually implies "don't ask me".
        // However, making it useful for scripts means falling back to sane defaults is often desired if default_settings wasn't explicitly passed but we want automation.
        // Let's assume if parameters are missing, we default them for convenience, OR check logic.
        // User requirements say: "Argument missing -> Interactive". "Non-interactive -> Error or Default".
        // Let's default for missing fields to be helpful.
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
        };
    }

    // Interactive Mode
    println!("{}", t("welcome"));
    if cli.cn.is_none() && cli.sans.is_none() && cli.days.is_none() {
        println!("{}", t("interactive_mode"));
    }

    let cn = match cli.cn {
        Some(v) => v,
        None => Text::new(&t("enter_cn"))
            .with_default(&default_cn)
            .prompt()
            .unwrap_or_else(|_| process::exit(0)),
    };

    let sans = match cli.sans {
        Some(v) => v,
        None => {
            let input = Text::new(&t("enter_san"))
                .with_default("127.0.0.1")
                .prompt()
                .unwrap_or_else(|_| process::exit(0));
            input.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    };

    let days = match cli.days {
        Some(v) => v,
        None => CustomType::<i64>::new(&t("enter_days"))
            .with_default(default_days)
            .with_error_message("Please enter a valid number")
            .prompt()
            .unwrap_or_else(|_| process::exit(0)),
    };

    CertOptions {
        cn,
        sans,
        validity_days: days,
    }
}
