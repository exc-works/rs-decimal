---
locale: ja
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

正確な結果のみを許可します。

```rust
Unnecessary = 6
```

## 引数、戻り値、通常の使い方

整数の判別値は 6 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。ゼロの桁を捨てても正確ですが、ゼロ以外の桁を 1 つでも捨てると RoundUnnecessary を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
