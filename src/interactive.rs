use crate::cert::{CertOptions, KeyType};
use crate::cli::Cli;
use crate::i18n::t;
use inquire::{Confirm, CustomType, Select, Text};
use std::process;

pub fn resolve_options(cli: Cli) -> CertOptions {
    if cli.cmdlist {
        println!("{}", t("cmdlist_header"));
        println!("--full: Ask for Country, State, City, Org, OrgUnit");
        println!("--all: Ask for KeyType and all other fields");
        println!("--default_settings: Generate with defaults");
        println!("--non-interactive: Fail on missing args (or use defaults)");
        process::exit(0);
    }

    let default_cn = "localhost".to_string();
    let default_sans = vec!["127.0.0.1".to_string()];
    let default_days = 365;
    let default_key_type = KeyType::Ecdsa;

    // Direct return if default_settings is requested
    if cli.default_settings {
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
            country: None,
            state: None,
            city: None,
            organization: None,
            org_unit: None,
            key_type: default_key_type,
        };
    }

    // Determine mode
    let is_all = cli.all;
    let is_full = cli.full || is_all;

    // Non-interactive fallback
    // If non-interactive is set, we use provided args or defaults, skipping prompts.
    if cli.non_interactive {
        return CertOptions {
            cn: cli.cn.unwrap_or(default_cn),
            sans: cli.sans.unwrap_or(default_sans),
            validity_days: cli.days.unwrap_or(default_days),
            country: None, // Defaults to None for non-interactive simple/full unless we parse more args (which we don't have yet)
            // Wait, if user wants to set C/ST/L via CLI arguments, we didn't add those args to Clap yet!
            // Requirement said: "Hybrid Input... 1. Command Line Params... 2. Interactive".
            // Implementation Plan only said "Add flags --full --all".
            // It didn't explicitly say "Add CLI args for Country, State etc".
            // However, to obtain "Full" mode via CLI arguments, we intuitively should support them.
            // But the user request specifically asked for interactive flows.
            // Let's stick to what we have. If non-interactive, those fields will be None unless we add args.
            // For now, let's keep them None in non-interactive if args aren't there.
            state: None,
            city: None,
            organization: None,
            org_unit: None,
            key_type: default_key_type,
        };
    }

    // Interactive Mode
    println!("{}", t("welcome"));
    if !is_full && cli.cn.is_none() && cli.sans.is_none() && cli.days.is_none() {
        println!("{}", t("interactive_mode"));
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

    // 2. Full Mode Fields (Optional)
    let mut country = None;
    let mut state = None;
    let mut city = None;
    let mut organization = None;
    let mut org_unit = None;

    if is_full {
        country = prompt_optional(&t("enter_country"));
        state = prompt_optional(&t("enter_state"));
        city = prompt_optional(&t("enter_city"));
        organization = prompt_optional(&t("enter_org"));
        org_unit = prompt_optional(&t("enter_org_unit"));
    }

    // 3. All Mode Fields
    let mut key_type = default_key_type;
    if is_all {
        let options = vec!["RSA", "ECDSA", "Ed25519"];
        let ans = Select::new(&t("select_key_type"), options)
            .prompt()
            .unwrap_or_else(|_| process::exit(0));

        key_type = match ans {
            "RSA" => KeyType::Rsa,
            "ECDSA" => KeyType::Ecdsa,
            "Ed25519" => KeyType::Ed25519,
            _ => KeyType::Rsa,
        };
    }

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
