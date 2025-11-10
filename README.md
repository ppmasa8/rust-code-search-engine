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

## 機能

- CLI: `apps/cli` からクロール／検索／重フィクスチャ生成コマンドを提供し、インデックス前処理や負荷再現を行える。
- API サーバ: `apps/api` が Axum ベースの `/search` を公開し、検索結果（パス・スコア・スニペット）を JSON で返す。
- ワーカー: `apps/worker` が並列ジョブで検索＋フィクスチャ生成を連続実行し、バックグラウンド負荷をシミュレートする。
- ライブラリ郡: `crates/` 以下の crawler/indexer/query/search/embedding/storage/contracts/testing が責務ごとに分かれ、インデックス構築やクエリ解析、疑似埋め込みスコアリング、インメモリストア、重テスト用ユーティリティを提供する。
