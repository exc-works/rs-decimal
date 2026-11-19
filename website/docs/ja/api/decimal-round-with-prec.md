---
locale: ja
source_revision: d6f697a3d962962b5c45eb345607fbc6629bac0f2d7f0f5c403da5649a7d1d4a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

指定した小数点以下の桁数で最近接偶数丸めを行います。

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 引数

`prec: isize` — 残す小数点以下の桁数（整数）。

## 戻り値

既定のゼロを渡した場合も初期化済みの Decimal を返します。入力を借用し、精度を上げると末尾にゼロを追加します。

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 別の丸めモードは [`rescale`](./decimal-rescale) で指定できます。

## エラー

負の精度で返すエラーは `InvalidPrecision`. JavaScript は対応する例外を送出します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
