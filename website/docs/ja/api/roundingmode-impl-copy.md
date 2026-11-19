---
locale: ja
source_revision: ce0e6eba90e03ef0b3ea77c71d25c9216e58acb759f414a585091bbcd0fa1215
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::impl Copy
---
# RoundingMode::impl Copy

丸めモードに対して Rust の Copy 実装を使います。

```rust
impl Copy for RoundingMode
```

## 引数、戻り値、通常の使い方

RoundingMode は小さな Copy 列挙型です。コピーは選択した規則を保持し、等価性はバリアントを比較し、Debug は名前を表示します。この例はネイティブ Rust のトレイト構文を検証します。ブラウザーのモードは不変の数値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-copy" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
