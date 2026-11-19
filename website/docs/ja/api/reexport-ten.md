---
locale: ja
source_revision: 7a73d9c8a38938796f2eacbe88712430716dafd16a136172a660bfe237d37db2
translation_provenance: en-US-japanese-reviewed
api_id: reexport::ten
---
# reexport::ten

rs_decimal クレートルートから ten をインポートします。

```rust
pub use decimal::ten as ten
```

## 通常の使い方と意味

rs_decimal::ten を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
