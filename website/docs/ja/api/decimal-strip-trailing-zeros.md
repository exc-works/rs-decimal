---
locale: ja
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

数値を変えず、小数部分の不要な末尾のゼロを除去します。

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 引数、戻り値、通常の使い方

引数なし。独立した Decimal を返します。スケールが正の間だけ 10 の因数を除去するため、1200 のような整数のゼロは失われません。数値ゼロはすべてスケール 0 の初期化済みゼロになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
