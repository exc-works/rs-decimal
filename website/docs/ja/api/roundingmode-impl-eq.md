---
locale: ja
source_revision: d588c3aa025efc49b40f1c2c9f6f76c0ff1df249830b32d566fb6a6a5794cdc5
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::impl Eq
---
# RoundingMode::impl Eq

丸めモードに対して Rust の Eq 実装を使います。

```rust
impl Eq for RoundingMode
```

## 引数、戻り値、通常の使い方

RoundingMode は小さな Copy 列挙型です。コピーは選択した規則を保持し、等価性はバリアントを比較し、Debug は名前を表示します。この例はネイティブ Rust のトレイト構文を検証します。ブラウザーのモードは不変の数値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-eq" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
