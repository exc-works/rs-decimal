---
locale: ja
source_revision: 893ae85fdbe2ce4fb0a97cdee41fd9675ed135240e7df9b22b449b50279fb13c
translation_provenance: en-US-japanese-reviewed
api_id: reexport::Decimal
---
# reexport::Decimal

rs_decimal クレートルートから Decimal をインポートします。

```rust
pub use decimal::Decimal as Decimal
```

## 通常の使い方と意味

rs_decimal::Decimal を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
