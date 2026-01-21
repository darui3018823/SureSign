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
- **Config File**: `.suresign.json` for persistent settings.

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

### Custom Output Path
```bash
suresign --output ./certs --name mycert --default_settings
```

## CLI Options

| Flag | Description |
|------|-------------|
| `--cn`, `-c` | Common Name (e.g., `myserver.local`) |
| `--sans`, `-s` | Subject Alternative Names (comma-separated) |
| `--days`, `-d` | Validity in days |
| `--output`, `-o` | Output directory |
| `--name`, `-n` | Output file name (without extension) |
| `--pfx-password` | PFX password |
| `--full` | Enable Full Mode (DN fields) |
| `--all` | Enable All Mode (KeyType, etc.) |
| `--country` | Country Name (2 letter code) |
| `--state` | State or Province Name |
| `--city` | Locality Name |
| `--org` | Organization Name |
| `--org-unit` | Organizational Unit Name |
| `--key-type` | Key algorithm: `rsa`, `ecdsa`, `ed25519` |
| `--default_settings` | Use all defaults |
| `--non-interactive` | Skip prompts |
| `--yes`, `-y` | Skip overwrite confirmation |
| `--verbose`, `-v` | Show verbose output |
| `--quiet`, `-q` | Suppress non-essential output |
| `--config` | Path to config file |
| `--cmdlist` | Show available commands |

## Config File

Create `.suresign.json` in the current directory or home directory:

```json
{
  "cn": "myserver.local",
  "sans": ["127.0.0.1", "192.168.1.1"],
  "days": 365,
  "country": "JP",
  "state": "Tokyo",
  "city": "Shibuya",
  "org": "My Company",
  "org_unit": "Dev Team",
  "key_type": "ecdsa",
  "output": "./certs",
  "name": "server"
}
```

CLI arguments take precedence over config file values.

## License

BSD-2-Clause. See [LICENSE](./license).

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for version history.
