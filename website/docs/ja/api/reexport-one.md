---
locale: ja
source_revision: f5ea64d7e7a6135acc2f55f3a40411c6d9222e20ac49981dab203cb401fa6651
translation_provenance: en-US-japanese-reviewed
api_id: reexport::one
---
# reexport::one

rs_decimal クレートルートから one をインポートします。

```rust
pub use decimal::one as one
```

## 通常の使い方と意味

rs_decimal::one を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
