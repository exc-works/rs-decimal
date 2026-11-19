---
locale: ja
source_revision: ef3aa99db04ca1bd7e821c4d4bf0be02f9db3337ae118e60dc96d5695ab892b9
translation_provenance: en-US-japanese-reviewed
api_id: reexport::hundred
---
# reexport::hundred

rs_decimal クレートルートから hundred をインポートします。

```rust
pub use decimal::hundred as hundred
```

## 通常の使い方と意味

rs_decimal::hundred を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
