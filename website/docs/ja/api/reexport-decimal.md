---
locale: ja
source_revision: 6976205af89f0a7bb675a6147933c2fe624e8c9d96703f7f354c0cb5ca52f755
translation_provenance: en-US-japanese-reviewed
api_id: reexport::Decimal
---
# reexport::Decimal

rs_decimal クレートルートから Decimal をインポートします。

```rust
pub use decimal::Decimal as Decimal
```

## 通常の使い方と意味

rs_decimal::Decimal を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
