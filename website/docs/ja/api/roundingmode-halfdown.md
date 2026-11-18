---
locale: ja
source_revision: be3c0e366eb3a51881c8080b2a8de1d5f2faa06f8be431e48c0342e9c8d4c0e5
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

最も近い値へ丸め、中間ではゼロ方向を選びます。

```rust
HalfDown = 4
```

## 引数、戻り値、通常の使い方

整数の判別値は 4 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。Down と Up はゼロを基準にした方向であり、負または正の無限大ではありません。Half 系はちょうど中間の場合だけ異なります。Floor という列挙子はないため、その方向には floor_with_prec を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
