---
locale: ja
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

指定した小数点以下の桁数で最近接偶数丸めを行います。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 引数

`prec: isize` — 小数点以下の桁数。bridge は JavaScript の整数値を受け取ります。

## 戻り値

既定のゼロを渡した場合も初期化済みの Decimal を返します。入力を借用し、精度を上げると末尾にゼロを追加します。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 別の丸めモードは [`rescale`](./decimal-rescale) で指定できます。

## エラー

負の精度で返すエラーは `InvalidPrecision`. bridge は対応する例外を送出します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
