---
locale: ja
source_revision: 8e2ccd4d5de92023ccad23e3b1ce732438fad5dab0ed6652057ad04ade8ca4a5
translation_provenance: en-US-japanese-reviewed
api_id: reexport::hundred
---
# reexport::hundred

rs_decimal クレートルートから hundred をインポートします。

```rust
pub use decimal::hundred as hundred
```

## 通常の使い方と意味

rs_decimal::hundred を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
