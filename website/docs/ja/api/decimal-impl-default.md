---
locale: ja
source_revision: 23ce03e01f179e79048016168712d7a3101de4dd03467bd8aefb98f6fd47b269
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Rust の Default で未初期化の数値ゼロを作成します。

```rust
impl Default for Decimal
```

## 通常の使い方と意味

Default は構築したゼロと数値として等しいものの、is_nil() は true です。JavaScript は Decimal.default() を提供します。このページは Rust トレイト自体を説明します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
