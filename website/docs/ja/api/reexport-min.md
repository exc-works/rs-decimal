---
locale: ja
source_revision: f747bad3502ad5af9ea13bde6cf07e23bd90f835c390a356a17c381617af16c5
translation_provenance: en-US-japanese-reviewed
api_id: reexport::min
---
# reexport::min

rs_decimal クレートルートから min をインポートします。

```rust
pub use decimal::min as min
```

## 通常の使い方と意味

rs_decimal::min を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
