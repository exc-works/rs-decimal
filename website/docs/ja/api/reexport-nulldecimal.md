---
locale: ja
source_revision: f04757565931a8e8fa648872ee9153875f735f45247f70875bf4fa04dbb2585d
translation_provenance: en-US-japanese-reviewed
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

rs_decimal クレートルートから NullDecimal をインポートします。

```rust
pub use decimal::NullDecimal as NullDecimal
```

## 通常の使い方と意味

rs_decimal::NullDecimal を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
