---
locale: ja
source_revision: 24d7fa4a2221c5f03dbc4893af4fe9b8c3b42496692a1b9ed6c8a03465745caf
translation_provenance: en-US-japanese-reviewed
api_id: reexport::max
---
# reexport::max

rs_decimal クレートルートから max をインポートします。

```rust
pub use decimal::max as max
```

## 通常の使い方と意味

rs_decimal::max を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
