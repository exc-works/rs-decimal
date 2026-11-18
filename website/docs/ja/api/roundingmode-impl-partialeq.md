---
locale: ja
source_revision: d358fea1722186e0df2e76e232b3793bff18d2e3a10cb6ef0ae0a724a40da40b
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

丸めモードに対して Rust の PartialEq 実装を使います。

```rust
impl PartialEq for RoundingMode
```

## 引数、戻り値、通常の使い方

RoundingMode は小さな Copy 列挙型です。コピーは選択した規則を保持し、等価性はバリアントを比較し、Debug は名前を表示します。この例はネイティブ Rust のトレイト構文を検証します。ブラウザーのモードは不変の数値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="ja" />

## ブラウザーの制約

整数引数は wasm32 の範囲内である必要があります。ブラウザーはさらに[リソースポリシー](../resource-policy)を適用し、指定スケール、係数の増大、呼び出し回数を制限します。リソースエラーと数値ライブラリのエラーは区別されます。借用メソッドは入力を消費しません。Rust 専用のトレイト構文はネイティブで検証するものとして明示します。

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
