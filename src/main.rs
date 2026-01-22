mod cert;
mod cli;
mod config;
mod i18n;
mod interactive;

use crate::config::Config;
use crate::i18n::t;
use anyhow::{Context, Result};
use console::Style;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let args = cli::parse();

    // Store verbose/quiet flags before moving args
    let verbose = args.verbose;
    let quiet = args.quiet;
    let yes = args.yes;

    // Load config file
    let config = if let Some(ref config_path) = args.config {
        Config::load(config_path).ok().flatten()
    } else {
        Config::load_default()
    };

    // Merge config with CLI args (CLI takes precedence)
    let output_dir: Option<PathBuf> = args.output.clone().or_else(|| {
        config
            .as_ref()
            .and_then(|c| c.output.as_ref().map(PathBuf::from))
    });

    let file_name = args
        .name
        .clone()
        .or_else(|| config.as_ref().and_then(|c| c.name.clone()))
        .unwrap_or_else(|| "server".to_string());

    // Log config file usage
    if verbose {
        if config.is_some() {
            println!("  Config file: loaded");
        } else {
            println!("  Config file: none");
        }
    }

    let opts = interactive::resolve_options_with_config(args, config);

    if !quiet {
        println!("{}", t("generating"));
    }

    if verbose {
        println!("  CN: {}", opts.cn);
        println!("  SANs: {:?}", opts.sans);
        println!("  Days: {}", opts.validity_days);
        println!("  Key Type: {:?}", opts.key_type);
        if opts.pfx_password.is_empty() {
            println!("  PFX Password: (none)");
        } else {
            println!("  PFX Password: (set)");
        }
    }

    let generated = cert::generate_cert(opts).context("Failed to generate certificate")?;

    // Determine output directory
    let output_path = output_dir.unwrap_or_else(|| Path::new(".").to_path_buf());

    // Create output directory if it doesn't exist
    if !output_path.exists() {
        fs::create_dir_all(&output_path).context("Failed to create output directory")?;
    }

    let key_path = output_path.join(format!("{}.key", file_name));
    let crt_path = output_path.join(format!("{}.crt", file_name));
    let pem_path = output_path.join(format!("{}.pem", file_name));
    let pfx_path = output_path.join(format!("{}.pfx", file_name));

    // Check for existing files and prompt for overwrite
    let paths = [&key_path, &crt_path, &pem_path, &pfx_path];
    let existing_files: Vec<_> = paths.iter().filter(|p| p.exists()).collect();

    if !existing_files.is_empty() && !yes {
        println!(
            "{}",
            Style::new().yellow().apply_to(t("files_exist_warning"))
        );
        for file in &existing_files {
            println!("  - {}", file.display());
        }
        print!("{} ", t("overwrite_prompt"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("{}", t("aborted"));
            return Ok(());
        }
    }

    // Write files
    fs::write(&key_path, &generated.key_pem).context("Failed to write key file")?;
    fs::write(&crt_path, &generated.cert_pem).context("Failed to write crt file")?;

    // server.pem = key + cert bundle
    let bundle = format!("{}\n{}", generated.key_pem, generated.cert_pem);
    fs::write(&pem_path, bundle).context("Failed to write pem file")?;

    fs::write(&pfx_path, &generated.pfx).context("Failed to write pfx file")?;

    if !quiet {
        let success_style = Style::new().green().bold();
        let path_style = Style::new().cyan();

        println!("{}", success_style.apply_to(t("success")));
        println!(
            "{} {}",
            t("saved_to"),
            path_style.apply_to(fs::canonicalize(&output_path)?.display())
        );
        println!("  - {}", path_style.apply_to(key_path.display()));
        println!("  - {}", path_style.apply_to(crt_path.display()));
        println!("  - {}", path_style.apply_to(pem_path.display()));
        println!("  - {}", path_style.apply_to(pfx_path.display()));
    }

    Ok(())
}
