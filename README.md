# SureSign

A pure Rust self-signed certificate generator with interactive and CLI modes.

![License](https://img.shields.io/badge/license-BSD--2--Clause-blue)

## Features

- **Pure Rust**: No OpenSSL dependency. Uses `rcgen` and `p12`.
- **Hybrid Input**: CLI arguments + Interactive prompts.
- **Modes**:
  - **Simple** (default): CN, SAN, Validity days.
  - **Full** (`--full`): Adds Country, State, City, Org, OrgUnit.
  - **All** (`--all`): Adds KeyType selection (RSA/ECDSA/Ed25519).
- **I18n**: Auto-detects system language (English/Japanese).
- **Output**: `.key`, `.crt`, `.pem`, `.pfx`

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## Usage

### Interactive Mode (Simple)
```bash
suresign
```

### Full Mode
```bash
suresign --full
```

### All Mode (Expert)
```bash
suresign --all
```

### Non-Interactive (Script-friendly)
```bash
suresign --cn myserver.com --days 365 --non-interactive
```

### Quick Defaults
```bash
suresign --default_settings
```

## CLI Options

| Flag | Description |
|------|-------------|
| `--cn` | Common Name (e.g., `myserver.local`) |
| `--sans` | Subject Alternative Names (comma-separated) |
| `--days` | Validity in days |
| `--full` | Enable Full Mode (DN fields) |
| `--all` | Enable All Mode (KeyType, etc.) |
| `--default_settings` | Use all defaults |
| `--non-interactive` | Skip prompts |
| `--cmdlist` | Show available commands |

## License

BSD-2-Clause. See [LICENSE](./license).
