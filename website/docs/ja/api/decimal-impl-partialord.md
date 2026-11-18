---
locale: ja
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

Rust の半順序で小数を比較します。

```rust
impl PartialOrd for Decimal
```

## 通常の使い方と意味

すべての Decimal 値は比較可能なので、partial_cmp は Some(Ordering) を返します。既定のゼロと構築したゼロも比較でき、スケールの違いによって None になることはありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
