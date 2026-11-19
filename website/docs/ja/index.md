---
locale: ja
source_revision: 92be186c5740ac6d22b3ca742b8ced1f5ab7822e9ab835f05db11a750bdadf79
aside: false
---
# rs-decimal

rs-decimal は Rust で十進数を扱うためのライブラリです。価格の足し算や、単価と数量からの小計の計算、レシートに表示する小数桁の指定に使えます。

## レシートの金額を計算してみよう

単価 4.50 のノートを 2 冊買い、送料 1.25 を加えると、支払額は 10.25 です。注文の単価を読み取ってからレシートに印字するまで、同じ Decimal の値を使えます。

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

<div class="ref-actions">

[最初の注文を計算する](/ja/guide)
[ブラウザーで数量を変えてみる](/ja/playground)

</div>
