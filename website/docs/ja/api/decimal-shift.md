---
locale: ja
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
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

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
