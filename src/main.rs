mod cli;
mod interactive;
mod cert;
mod i18n;

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use console::Style;
use crate::i18n::t;

fn main() -> Result<()> {
    // Force init of i18n (it's lazy but good to ensure it detects early if needed)
    // Actually our lazy_static handles it on first call.

    let args = cli::parse();
    let opts = interactive::resolve_options(args);

    println!("{}", t("generating"));

    let generated = cert::generate_cert(opts).context("Failed to generate certificate")?;

    // Save files
    let output_dir = "."; // Current directory or configurable? Req says "save path prominently"
    
    let key_path = Path::new(output_dir).join("server.key");
    let crt_path = Path::new(output_dir).join("server.crt");
    let pem_path = Path::new(output_dir).join("server.pem"); // Bundle
    let pfx_path = Path::new(output_dir).join("server.pfx");

    fs::write(&key_path, &generated.key_pem).context("Failed to write server.key")?;
    fs::write(&crt_path, &generated.cert_pem).context("Failed to write server.crt")?;
    
    // server.pem = cert + key
    let bundle = format!("{}\n{}", generated.key_pem, generated.cert_pem);
    fs::write(&pem_path, bundle).context("Failed to write server.pem")?;

    fs::write(&pfx_path, &generated.pfx).context("Failed to write server.pfx")?;

    let success_style = Style::new().green().bold();
    let path_style = Style::new().cyan();

    println!("{}", success_style.apply_to(t("success")));
    println!("{} {}", t("saved_to"), path_style.apply_to(fs::canonicalize(output_dir)?.display()));
    println!("  - {}", path_style.apply_to(key_path.display()));
    println!("  - {}", path_style.apply_to(crt_path.display()));
    println!("  - {}", path_style.apply_to(pem_path.display()));
    println!("  - {}", path_style.apply_to(pfx_path.display()));

    Ok(())
}
