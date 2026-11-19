---
locale: ja
source_revision: d9e6c46654b05ae244bd56f7035b17e55d6f30ce2bee3665fad9be992491dd6d
translation_provenance: en-US-japanese-reviewed
api_id: reexport::zero
---
# reexport::zero

rs_decimal クレートルートから zero をインポートします。

```rust
pub use decimal::zero as zero
```

## 通常の使い方と意味

rs_decimal::zero を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
