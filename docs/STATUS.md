# 進捗と引き継ぎ

最終更新: 2026-11-29
最終コミット: `c3b54b5 phase 0: add start screen and quit handling`

実装計画は `docs/PLAN.md`。このファイルは「今どこまで終わっていて、次に何をするか」を新しいセッションに引き継ぐためのもの。

## 現在のフェーズ

**Phase 1（ドメイン）開始前。** Phase 0（準備）は完了している。

## Phase 0 で完成したもの

- パッケージ構成: edition 2024、`src/lib.rs` + `src/main.rs`
- lint: `Cargo.toml` の `[lints]`
  - `unsafe_code = "forbid"`
  - clippy の `all` + `pedantic`（`module_name_repetitions` と `must_use_candidate` は allow）
  - `cargo clippy --all-targets -- -D warnings` が通る
- ログ: `src/logging.rs`
  - `init()` が `WorkerGuard` を返し、`main` が保持する
  - 保存先は OS のデータディレクトリ配下 `.../situation-puzzle-tui/logs/`
  - 日次ローテーション、`RUST_LOG` でレベル変更（既定 `info`）
  - `tracing_appender::rolling::Builder` を使い、初期化失敗は panic せず `LoggingError` を返す
  - タイムスタンプとファイル名は UTC
- TUI: `src/ui.rs` と `src/lib.rs`
  - `run()` が `ratatui::try_init()` → イベントループ → `ratatui::restore()`
  - 起動画面は枠・タイトル・中央の案内文
  - `q` か `Esc` で終了（`KeyEventKind::Press` のみ反応）
  - 80×24 のスナップショットテスト（`TestBackend` + `insta`）
- 依存: `directories`、`thiserror`、`tracing`、`tracing-appender`、`tracing-subscriber`（`env-filter`）、`ratatui`、`crossterm`。dev 依存は `insta`

## ファイルの役割

| ファイル | 役割 |
| --- | --- |
| `src/main.rs` | ログ初期化と `run()` の呼び出し、失敗時の終了 |
| `src/lib.rs` | `run()`、端末イベントループ、モジュール宣言 |
| `src/ui.rs` | 起動画面の描画とスナップショットテスト |
| `src/logging.rs` | ファイルログの初期化と `LoggingError` |

## 決めたこと・注意点

- コード中のコメントとドキュメントコメントは英語。`docs/` と `AGENTS.md` は日本語
- UI 文字列はまだハードコード（`"Press q or Esc to quit"`）。Phase 2 で `i18n` の `Text` enum に移す
- Ctrl+C は未対応。raw mode では SIGINT が無効なので、Phase 2 のキーマップで `KeyModifiers::CONTROL` + `Char('c')` を足す
- `main` のエラー表示は `Display` のみで、原因のチェーンは出ない。`color-eyre` 導入時にまとめて直す
- ログは UTC 基準。JST では日付が前日になることがある
- `cargo-insta` は未インストール。スナップショットの更新は `INSTA_UPDATE=always cargo test` で行う
- スナップショット（`.snap`）はコミットする。`*.snap.new` は `.gitignore` 済み

## 確認コマンド

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

`cargo run` は `q` か `Esc` で終了する。

## 次にやること（Phase 1）

`domain` モジュールを作り、IO なしのゲーム進行を型で表す。

- `Puzzle`、`PuzzleId`（newtype）、`Language`、`Source`
- `GameSession`、`Turn`、`Verdict`
- ゲームの状態（Playing / Solved / GaveUp）を enum で表す
- 終了後の質問はエラーにする
- 単体テストで「質問数の数え方」「ギブアップ後は質問できない」などを表現する
- `domain` から ratatui・tokio・reqwest に依存しない

## 未決事項

`docs/PLAN.md` の「未決事項」を参照。Phase 1 の範囲では、`Puzzle` のフィールド構成（言語・出典・タグ・作成日時）をどこまで最初から持たせるかを決める必要がある。