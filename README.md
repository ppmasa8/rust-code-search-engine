# Rust Code Search Engine Workspace

このモノレポは、Tantivy と ONNX ベースのコード検索エンジン用に設計された参考構成です。

## ディレクトリ構成

```
.
├── apps/          # エンドユーザ向け実行バイナリ
├── crates/        # 再利用可能なライブラリクレート群
├── data/          # コーパスやテスト用データ
├── docs/          # アーキテクチャ・開発ドキュメント
├── scripts/       # CI/開発で使うユーティリティ
├── specs/         # 機能仕様やスキーマ定義
├── tests/         # E2E/統合テストとフィクスチャ
└── .github/       # CI/CD ワークフロー
```

## ビルドとテスト

```bash
cargo build --workspace
cargo test --workspace -- --ignored
```
