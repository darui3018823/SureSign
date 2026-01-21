# SureSign へのコントリビューション

SureSignへのコントリビューションに興味をお持ちいただき、ありがとうございます。私たちはコミュニティからの貢献を歓迎します。このドキュメントは、プロジェクトへの貢献方法についてのガイドラインと手順を提供します。

## 行動規範

このプロジェクトは[行動規範](./Code_of_Conduct_ja.md)に基づいています。プロジェクトに参加することで、その条件に従うことに同意します。

## コントリビューションの方法

### バグレポート

バグを見つけた場合、以下の情報を含むイシューを作成してください：

- **説明**: バグについて明確かつ簡潔な説明
- **再現手順**: バグを再現する手順
- **期待動作**: 何が起こるはずだったか
- **実際の動作**: 実際に何が起こったか
- **環境**: OS、Rustバージョン、その他の関連情報
- **スクリーンショット/ログ**: 該当する場合、エラーメッセージやログを含める

### 機能提案

新しい機能や改善についての提案を歓迎します：

- **説明**: 機能について明確かつ簡潔な説明
- **動機**: この機能が有用だと考える理由
- **代替案**: 検討した代替アプローチ
- **追加情報**: その他の関連情報

### プルリクエストの提出

1. **リポジトリをフォークする**: GitHubの「Fork」ボタンをクリック

2. **フォークをクローンする**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/SureSign.git
   cd SureSign
   ```

3. **フィーチャーブランチを作成する**:
   ```bash
   git checkout -b feat/your-feature-name
   ```
   またはバグ修正の場合:
   ```bash
   git checkout -b fix/your-bug-fix
   ```

4. **変更を実装する**: 機能またはバグ修正を実装

5. **コミットメッセージ規約に従う**:
   [Conventional Commits](https://www.conventionalcommits.org/)に従います：
   - `feat: 新機能を追加`
   - `fix: バグを修正`
   - `docs: ドキュメントを更新`
   - `refactor: コードをリファクタリング`
   - `test: テストを追加`
   - `chore: 依存関係を更新`

6. **変更をテストする**:
   ```bash
   cargo test
   cargo build --release
   ```

7. **フォークにプッシュする**:
   ```bash
   git push origin feat/your-feature-name
   ```

8. **プルリクエストを作成する**: GitHubで「New Pull Request」をクリックして、以下を記載：
   - 変更を説明する明確なタイトル
   - 何を変更したかと理由についての詳細説明
   - 関連するイシューへの参照（ある場合）

## 開発環境セットアップ

### 前提条件

- Rust 1.70以上
- Git

### ソースからビルド

```bash
git clone https://github.com/darui3018823/SureSign.git
cd SureSign
cargo build --release
```

### テストの実行

```bash
cargo test
```

### ドキュメントの生成

```bash
cargo doc --open
```

## プロジェクト構成

```
SureSign/
├── src/
│   ├── main.rs           # アプリケーション入口
│   ├── cert.rs           # 証明書生成と処理
│   ├── cli.rs            # コマンドラインインターフェース
│   ├── i18n.rs           # 国際化
│   └── interactive.rs    # インタラクティブモード
├── scripts/              # ビルドとユーティリティスクリプト
├── Cargo.toml            # プロジェクトマニフェスト
└── README.md             # プロジェクトドキュメント
```

## コードスタイル

- Rustの規約とベストプラクティスに従う
- `cargo fmt`でコードをフォーマット
- `cargo clippy`で一般的な間違いをチェック

```bash
cargo fmt
cargo clippy --all-targets
```

## プルリクエストレビュープロセス

1. PRはプロジェクトメンテナーによってレビュー
2. 変更を要求されたり、質問されたりする場合がある
3. 承認されたら、PRはマージされる
4. 貢献は次のリリースでクレジットされる

## リリースプロセス

[セマンティックバージョニング](https://semver.org/)に従います：
- メジャーバージョン: 後方互換性のない API変更
- マイナーバージョン: 後方互換性のある新機能
- パッチバージョン: 後方互換性のあるバグ修正

## ご質問は？

以下の方法で：
- `question`ラベル付きのイシューを開く
- [contact@daruks.com](mailto:contact@daruks.com)までお問い合わせ

SureSignへのコントリビューションをお待ちしています！
