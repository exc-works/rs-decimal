---
locale: ja
source_revision: a9093dcd1b9fe39261e7248debd207a7c4986a0a263f3e970359020a9ba293a8
translation_provenance: en-US-japanese-reviewed
api_id: reexport::between
---
# reexport::between

rs_decimal クレートルートから between をインポートします。

```rust
pub use decimal::between as between
```

## 通常の使い方と意味

rs_decimal::between を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-between" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
