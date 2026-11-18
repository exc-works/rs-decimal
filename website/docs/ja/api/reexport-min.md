---
locale: ja
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
translation_provenance: en-US-japanese-reviewed
api_id: reexport::min
---
# reexport::min

rs_decimal クレートルートから min をインポートします。

```rust
pub use decimal::min as min
```

## 通常の使い方と意味

rs_decimal::min を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
