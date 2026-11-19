---
locale: ja
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

丸めモードに対して Rust の Debug 実装を使います。

```rust
impl Debug for RoundingMode
```

## 引数、戻り値、通常の使い方

RoundingMode は小さな Copy 列挙型です。コピーは選択した規則を保持し、等価性はバリアントを比較し、Debug は名前を表示します。この例はネイティブ Rust のトレイト構文を検証します。ブラウザーのモードは不変の数値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
