---
locale: ja
source_revision: 6d142adfffddbbc67008e17859ebe097a896ff7983bb306596fd170c8b867b28
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::shift
---
# Decimal::shift

小数に正確な 10 の累乗を掛けます。

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

places: isize/整数 number。正なら小数点を右へ、負なら左へ移動します。rescale とは異なり数値を変更します。ゼロ以外の値のシフトでスケールが i32::MAX を超えると、コアは Overflow を返します。ブラウザーでは増大やスケールの検査によって、割り当て前に別のリソースエラーとなる場合があります。shift(0) でも既定のゼロを初期化します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
