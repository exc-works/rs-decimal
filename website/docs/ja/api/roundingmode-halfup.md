---
locale: ja
source_revision: e0f36bf4a4b64a112b7758ba3b58ccfab48769ae252409c4b72638890e0e2a0e
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

最も近い値へ丸め、中間ではゼロから遠ざかる方向を選びます。

```rust
HalfUp = 3
```

## 引数、戻り値、通常の使い方

整数の判別値は 3 です。マジックナンバーではなく名前付きの値を使ってください。規則は正負の両方に適用されます。Down と Up はゼロを基準にした方向であり、負または正の無限大ではありません。Half 系はちょうど中間の場合だけ異なります。Floor という列挙子はないため、その方向には floor_with_prec を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
