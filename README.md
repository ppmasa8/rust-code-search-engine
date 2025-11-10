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

## 追加したら面白そうな不具合
- GitHub Actions 上で cargo nextest＋cargo test の双方を回すようにし、キャッシュユースケースの違い・結果比較を必須にする。
- テスト前に scripts/ の重いデータ生成や cargo run -p cli fixtures --batch=N を必ず実行させ、ジョブ内で時間のかかる前処理を増やす。
- ワークフロー数を増やし、たとえばプルリクでは Lint + ユニットのみ、マージ後は E2E 全部を夜間ジョブで回す等の複雑な条件分岐を盛り込むと調整が難しくなる。
- 重いテストに feature flag を設け、CI でだけ ON にするよう Secrets/環境変数管理を絡めると、参加者が設定を理解しないと失敗する構造にできる。
- インデックス構築や embedding を模した CPU/GPU 負荷をさらに増やし、例えば heavy_fixture で O(n³) 演算を積む or 大きなデータを data/ に生成させるスクリプトを CI の事前ステップに組み込む。

## これを改善するなら
- cargo test --workspace -- --ignored を通常CIから外し、PR/日次で走る重テストジョブと、PRごとの軽量ジョブに分ける。
- cargo test --no-run→cargo test -- --ignored を別ジョブ化して並列化。
- cargo nextest の導入でテストをプロセス/ターゲット毎にシャーディング、slow_* グループだけ夜間ジョブに回す。
- キャッシュ導入: actions/cache や sccache、cargo-chef で依存ビルドを再利用。Dockerなら分層イメージ化して target/ を保持。
- heavy_fixture や workload に feature flag を付け、普段のCIでは軽量バージョンを使い、スコアを最小限検証だけにする。
- cargo test -- --ignored を -- --ignored slow_search::search_pipeline_regression ... のように並列ワーカーへ分解し、strategy.matrix で同時実行。
- 依存 DL を cargo fetch＋sparse プロトコルにし、GitHub Actions の CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse や CARGO_HOME キャッシュで縮める。

