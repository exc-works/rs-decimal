---
locale: ja
source_revision: 9dbdcebee0aa8b717d5990e56743976e5e18fb73cbd32737b093a37a0ebe40de
sidebar: false
aside: false
---
# rs-decimal

rs-decimal は Rust で十進数を扱うためのライブラリです。価格の足し算や、単価と数量からの小計の計算、レシートに表示する小数桁の指定に使えます。

## レシートの金額を計算してみよう

単価 4.50 のノートを 2 冊買い、送料 1.25 を加えると、支払額は 10.25 です。注文の単価を読み取ってからレシートに印字するまで、同じ Decimal の値を使えます。

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

- [最初の注文を計算する](/ja/guide)
- [ブラウザーで数量を変えてみる](/ja/playground)
- [操作から API メソッドを探す](/ja/api/)
