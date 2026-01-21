# サポート

SureSignをご利用いただき、ありがとうございます。ここはヘルプとサポートを受けるための方法です。

## ドキュメント

- [README](./README.md) - プロジェクト概要とクイックスタート
- [CONTRIBUTING](./CONTRIBUTING_ja.md) - コントリビューションガイド
- [行動規範](./Code_of_Conduct_ja.md) - コミュニティ基準

## ヘルプの取得

### GitHubイシュー

バグや機能リクエストは[GitHub Issues](https://github.com/darui3018823/SureSign/issues)を使用してください：

1. 既存のイシューを確認して重複を避ける
2. 明確な説明、再現手順、環境詳細を提供
3. 必要に応じてエラーメッセージやログを含める

### メールサポート

その他のお問い合わせ：
- 一般的な質問: [contact@daruks.com](mailto:contact@daruks.com)
- セキュリティ問題: [security@daruks.com](mailto:security@daruks.com)

## よくある質問

### インストールに関する問題

**Q: ソースからSureSignをビルドするにはどうしたらいいですか？**

A: 以下のコマンドを実行してください：
```bash
git clone https://github.com/darui3018823/SureSign.git
cd SureSign
cargo build --release
```

**Q: システム要件は何ですか？**

A: 
- Rust 1.70以上
- ディスク容量 50 MB
- ランタイムの外部依存関係なし

### 使用方法に関する問題

**Q: 生成された証明書はどこに保存されますか？**

A: デフォルトでは、証明書は現在の作業ディレクトリに保存されます。コマンドライン引数を使用してカスタムパスを指定できます。

**Q: 本番環境でSureSignを使用できますか？**

A: SureSign v1.0.0は本番対応です。セキュリティドキュメントを確認し、証明書管理のベストプラクティスに従ってください。

## コミュニティ

- **ディスカッション**: GitHub Discussionsで一般的な質問やアイデアを投稿
- **プルリクエスト**: コントリビューションを歓迎します。[CONTRIBUTING_ja.md](./CONTRIBUTING_ja.md)を参照
- **ソーシャルメディア**: 関連プラットフォームでアップデートをフォロー

## イシュー報告

### 受け付けるイシュー

- 再現可能なステップを含むバグレポート
- 明確なユースケースを伴う機能リクエスト
- ドキュメント改善
- パフォーマンス改善
- セキュリティ脆弱性（プライベートで報告）

### イシューテンプレート

イシューを報告する際は、以下を含めてください：

1. **SureSignバージョン**: `suresign --version`
2. **OS および環境**: Windows/Linux/macOS、アーキテクチャ
3. **エラーメッセージ**: 完全なエラー出力
4. **再現手順**: 明確で番号付きのステップ
5. **期待動作**: 何が起こるべきか
6. **実際の動作**: 実際に何が起こったか

## レスポンスタイム

以下を目指しています：
- イシュー受信確認：48時間以内
- 初期評価提供：1週間以内
- 重大なバグの修正リリース：2週間以内

レスポンスタイムは複雑さとチーム利用可能性に基づいて異なる場合があります。

## ロードマップ

計画中の機能と改善については、[GitHub Projects](https://github.com/darui3018823/SureSign/projects)をご覧ください。

## クレジット

SureSignは以下で構築されています：
- [Rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap) - CLI解析
- [rcgen](https://github.com/rustls/rcgen) - 証明書生成
- [p12](https://github.com/tintinn/p12) - PKCS#12対応

## ライセンス

SureSignは[BSD 2-Clause License](../license)の下でライセンスされています。

---

SureSignをご利用いただきありがとうございます。皆様のご意見とコントリビューションを歓迎します。
