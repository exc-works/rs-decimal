---
locale: ja
source_revision: be26937ee4bcd1a30b10deb30b4ced946f0371e3015f1e537f8c85709084d801
translation_provenance: en-US-japanese-reviewed
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

丸めモードに対して Rust の Clone 実装を使います。

```rust
impl Clone for RoundingMode
```

## 引数、戻り値、通常の使い方

RoundingMode は小さな Copy 列挙型です。コピーは選択した規則を保持し、等価性はバリアントを比較し、Debug は名前を表示します。この例はネイティブ Rust のトレイト構文を検証します。ブラウザーのモードは不変の数値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="ja" />

## 関連メソッド

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
