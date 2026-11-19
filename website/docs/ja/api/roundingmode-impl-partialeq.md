---
locale: ja
source_revision: dce0eeed904ef266b7197b18fd411cf158683a2fe3f9074f19a11965111706e9
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

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
