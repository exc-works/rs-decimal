---
locale: ja
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# はじめに

お客様が単価 4.50 のノートを 2 冊注文しました。送料は 1.25 です。支払額を計算してレシートに表示してみましょう。

## 支払額を計算する

注文票どおりに `new_from_string("4.50")` で単価を読み取ります。`mul_exact` で数量の 2 を掛けると小計は 9.00 です。`add` で送料を足すと支払額は 10.25 になります。

<<< ../../../examples/web_getting_started.rs#calculate

## レシートに金額を表示する

小計に対して `string()` を呼ぶと "9"、`string_with_trailing_zeros()` を呼ぶと "9.00" になります。同じ金額ですが、後者は単価の小数点以下 2 桁を保つため、レシートにそのまま使えます。

<<< ../../../examples/web_getting_started.rs#print

[ブラウザーで数量を変えてみる](./playground) — 数量を 2 から 3 に変えて、もう一度実行してください。小計は 13.50、支払額は 14.75 になります。

## 手元で Rust の例を実行する

Rust 1.85 以降の環境で次のコマンドを実行すると、リポジトリを取得してサンプルを実行できます。端末に 10.25 と表示されます。

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
