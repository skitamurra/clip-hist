# fuzz-clip

クリップボード履歴をファジー検索で素早く呼び出すデスクトップアプリです。

## 概要

`fuzz-clip` は [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) (egui) を使って構築された、クリップボード履歴管理ツールです。過去にコピーした内容を一覧表示し、目的のテキストを素早く見つけて再利用できます。

## 必要環境

- [Rust](https://www.rust-lang.org/) (edition 2024)

## ビルド・実行

```bash
# ビルド
cargo build

# 実行
cargo run
```

## 技術スタック

| 項目 | 内容 |
|------|------|
| 言語 | Rust |
| GUI フレームワーク | [eframe](https://crates.io/crates/eframe) / [egui](https://crates.io/crates/egui) |

## ライセンス

MIT
