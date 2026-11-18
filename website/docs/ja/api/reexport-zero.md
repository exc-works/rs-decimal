---
locale: ja
source_revision: 099c3533a52ede06adc92af14955d2d2a20dadf66c185c385b9eda728b52b6ba
translation_provenance: en-US-japanese-reviewed
api_id: reexport::zero
---
# reexport::zero

rs_decimal クレートルートから zero をインポートします。

```rust
pub use decimal::zero as zero
```

## 通常の使い方と意味

rs_decimal::zero を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
