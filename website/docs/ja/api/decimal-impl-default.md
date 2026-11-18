---
locale: ja
source_revision: 9ab9d90f4fdbf97aeb5b036e4b2c138345a66dab28af2a920054edaddf0608dd
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Rust の Default で未初期化の数値ゼロを作成します。

```rust
impl Default for Decimal
```

## 通常の使い方と意味

Default は構築したゼロと数値として等しいものの、is_nil() は true です。bridge は Decimal.default() を提供します。このページは Rust トレイト自体を説明します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
