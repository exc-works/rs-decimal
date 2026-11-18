---
locale: ja
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
translation_provenance: en-US-japanese-reviewed
api_id: reexport::ten
---
# reexport::ten

rs_decimal クレートルートから ten をインポートします。

```rust
pub use decimal::ten as ten
```

## 通常の使い方と意味

rs_decimal::ten を使います。同じ項目の公開インポートパスであり、別の数値実装ではありません。JavaScript には Rust の use 構文がないため、対応する bridge のリファレンスを参照してください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
