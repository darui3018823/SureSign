use crate::cert::{CertOptions, KeyType};
use crate::cli::Cli;
use crate::i18n::t;
use inquire::{CustomType, Select, Text};
use std::process;

/// Parse key type string from CLI to KeyType enum
fn parse_key_type(s: &str) -> KeyType {
    match s.to_lowercase().as_str() {
        "rsa" => KeyType::Rsa,
        "ecdsa" => KeyType::Ecdsa,
        "ed25519" => KeyType::Ed25519,
        _ => KeyType::Ecdsa,
    }
}

pub fn resolve_options(cli: Cli) -> CertOptions {
    if cli.cmdlist {
        println!("{}", t("cmdlist_header"));
        println!("--full: Ask for Country, State, City, Org, OrgUnit");
        println!("--all: Ask for KeyType and all other fields");
        println!("--default_settings: Generate with defaults");
        println!("--non-interactive: Fail on missing args (or use defaults)");
        println!("--output, -o: Output directory");
        println!("--name, -n: Output file name (without extension)");
        println!("--pfx-password: Set PFX password");
        println!("--country, --state, --city, --org, --org-unit: DN fields");
        println!("--key-type: Key algorithm (rsa, ecdsa, ed25519)");
        println!("--verbose, -v: Show verbose output");
        println!("--quiet, -q: Suppress non-essential output");
        println!("--config: Path to config file");
        process::exit(0);
    }

    let default_cn = "localhost".to_string();
    let default_sans = vec!["127.0.0.1".to_string()];
    let default_days = 365;
    let default_key_type = KeyType::Ecdsa;

    // Parse key_type from CLI if provided
    let cli_key_type = cli.key_type.as_ref().map(|s| parse_key_type(s));

    // Direct return if default_settings is requested
    if cli.default_settings {
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
            country: cli.country,
            state: cli.state,
            city: cli.city,
            organization: cli.org,
            org_unit: cli.org_unit,
            key_type: cli_key_type.unwrap_or(default_key_type),
            pfx_password: cli.pfx_password.unwrap_or_default(),
        };
    }

    // Determine mode
    let is_all = cli.all;
    let is_full = cli.full || is_all;

    // Non-interactive fallback
    if cli.non_interactive {
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
            country: cli.country,
            state: cli.state,
            city: cli.city,
            organization: cli.org,
            org_unit: cli.org_unit,
            key_type: cli_key_type.unwrap_or(default_key_type),
            pfx_password: cli.pfx_password.unwrap_or_default(),
        };
    }

    // Interactive Mode
    if !cli.quiet {
        println!("{}", t("welcome"));
        if !is_full && cli.cn.is_none() && cli.sans.is_none() && cli.days.is_none() {
            println!("{}", t("interactive_mode"));
        }
    }

    // 1. Basic Fields
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
            input
                .split(',')
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

    // 2. Full Mode Fields - use CLI args or prompt
    let country = if is_full {
        cli.country.or_else(|| prompt_optional(&t("enter_country")))
    } else {
        cli.country
    };

    let state = if is_full {
        cli.state.or_else(|| prompt_optional(&t("enter_state")))
    } else {
        cli.state
    };

    let city = if is_full {
        cli.city.or_else(|| prompt_optional(&t("enter_city")))
    } else {
        cli.city
    };

    let organization = if is_full {
        cli.org.or_else(|| prompt_optional(&t("enter_org")))
    } else {
        cli.org
    };

    let org_unit = if is_full {
        cli.org_unit.or_else(|| prompt_optional(&t("enter_org_unit")))
    } else {
        cli.org_unit
    };

    // 3. All Mode Fields - use CLI args or prompt
    let key_type = if let Some(kt) = cli_key_type {
        kt
    } else if is_all {
        let options = vec!["RSA", "ECDSA", "Ed25519"];
        let ans = Select::new(&t("select_key_type"), options)
            .prompt()
            .unwrap_or_else(|_| process::exit(0));

        match ans {
            "RSA" => KeyType::Rsa,
            "ECDSA" => KeyType::Ecdsa,
            "Ed25519" => KeyType::Ed25519,
            _ => KeyType::Ecdsa,
        }
    } else {
        default_key_type
    };

    // PFX Password - use CLI arg or prompt in All mode
    let pfx_password = if let Some(pwd) = cli.pfx_password {
        pwd
    } else if is_all {
        prompt_optional(&t("enter_pfx_password")).unwrap_or_default()
    } else {
        String::new()
    };

    CertOptions {
        cn,
        sans,
        validity_days: days,
        country,
        state,
        city,
        organization,
        org_unit,
        key_type,
        pfx_password,
    }
}

fn prompt_optional(msg: &str) -> Option<String> {
    let input = Text::new(msg).prompt().unwrap_or_default();
    if input.trim().is_empty() {
        None
    } else {
        Some(input.trim().to_string())
    }
}
