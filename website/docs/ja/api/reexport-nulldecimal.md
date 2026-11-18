---
locale: ja
source_revision: 05a4d32c73f6c4d59c8e657272c093b1db00da738b0234c960526209d3e3196c
translation_provenance: en-US-japanese-reviewed
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

rs_decimal クレートルートから NullDecimal をインポートします。

```rust
pub use decimal::NullDecimal as NullDecimal
```

## 通常の使い方と意味

rs_decimal::NullDecimal を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
