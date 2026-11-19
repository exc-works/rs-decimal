---
locale: ja
source_revision: 29e91f626fd685fb0b5987fc104c1ba0e2752897194773305e3d64dcc1b16f1a
translation_provenance: en-US-japanese-reviewed
api_id: reexport::one
---
# reexport::one

rs_decimal クレートルートから one をインポートします。

```rust
pub use decimal::one as one
```

## 通常の使い方と意味

rs_decimal::one を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
