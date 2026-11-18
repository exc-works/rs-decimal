---
locale: ja
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

Rust の半順序で小数を比較します。

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## 通常の使い方と意味

すべての Decimal 値は比較可能なので、partial_cmp は Some(Ordering) を返します。既定のゼロと構築したゼロも比較でき、スケールの違いによって None になることはありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
