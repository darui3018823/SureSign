# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-01-22

### Added

- **CLI Extensions**
  - Output directory (`--output`, `-o`) and file name (`--name`, `-n`) customization
  - PFX password support (`--pfx-password`)
  - Overwrite confirmation with skip option (`--yes`, `-y`)
  - Full/All mode CLI arguments (`--country`, `--state`, `--city`, `--org`, `--org-unit`, `--key-type`)
  - Verbose (`--verbose`, `-v`) and quiet (`--quiet`, `-q`) output modes
  - Config file support (`--config`)

- **Config File Support**
  - `.suresign.json` configuration file support
  - Auto-detection in current and home directories
  - CLI arguments override config file values

- **i18n Improvements**
  - Externalized translations to JSON files (`locales/en.json`, `locales/ja.json`)
  - Embedded fallback translations for standalone binary

- **Code Quality**
  - 5 unit tests for certificate generation
  - Improved error handling

### Changed

- **Dependencies Updated**
  - `rcgen` 0.11 → 0.13 (breaking API changes)
  - `clap` 4.4 → 4.5
  - `inquire` 0.6 → 0.7
  - Added `serde`, `serde_json`, `dirs` crates

### Breaking Changes

- rcgen 0.13 API migration: `Certificate::from_params()` → `CertificateParams::self_signed()`
- `KeyPair::generate()` → `KeyPair::generate_for()`

## [1.0.0] - 2026-01-21

### Added

- Initial release
- Pure Rust self-signed certificate generator (no OpenSSL dependency)
- Hybrid input mode: CLI arguments + Interactive prompts
- Three modes: Simple, Full (`--full`), All (`--all`)
- Auto-detected system language (English/Japanese)
- Output formats: `.key`, `.crt`, `.pem`, `.pfx`
- Default settings mode (`--default_settings`)
- Non-interactive mode (`--non-interactive`)
