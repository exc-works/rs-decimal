---
locale: ja
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
translation_provenance: en-US-japanese-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

クレートルートから丸めモードをインポートします。

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 引数、戻り値、通常の使い方

rs_decimal::RoundingMode を使います。同じ列挙型であり、別の実装ではありません。ブラウザーは Rust のインポート構文を使わず、名前付きの RoundingMode オブジェクトを公開します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
