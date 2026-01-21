# SureSign

Pure Rust製の自己署名証明書発行ツール。対話モードとCLIモードの両方に対応。

![License](https://img.shields.io/badge/license-BSD--2--Clause-blue)

## 特徴

- **Pure Rust**: OpenSSL不要。`rcgen`と`p12`を使用。
- **ハイブリッド入力**: CLI引数 + 対話プロンプト。
- **モード**:
  - **Simple** (デフォルト): CN、SAN、有効期限。
  - **Full** (`--full`): 国、都道府県、市区町村、組織、部署を追加。
  - **All** (`--all`): 鍵の種類選択（RSA/ECDSA/Ed25519）を追加。
- **I18n**: システム言語を自動検出（日本語/英語）。
- **出力**: `.key`, `.crt`, `.pem`, `.pfx`

## インストール

```bash
cargo install --path .
```

または、ソースからビルド:

```bash
cargo build --release
```

## 使い方

### 対話モード（Simple）
```bash
suresign
```

### Fullモード
```bash
suresign --full
```

### Allモード（エキスパート）
```bash
suresign --all
```

### 非対話モード（スクリプト向け）
```bash
suresign --cn myserver.com --days 365 --non-interactive
```

### デフォルト設定で実行
```bash
suresign --default_settings
```

## CLIオプション

| フラグ | 説明 |
|--------|------|
| `--cn` | コモンネーム（例: `myserver.local`） |
| `--sans` | サブジェクト代替名（カンマ区切り） |
| `--days` | 有効期限（日数） |
| `--full` | Fullモードを有効化 |
| `--all` | Allモードを有効化 |
| `--default_settings` | すべてデフォルト値を使用 |
| `--non-interactive` | プロンプトをスキップ |
| `--cmdlist` | 利用可能なコマンドを表示 |

## ライセンス

BSD-2-Clause。[LICENSE](../license)を参照。
