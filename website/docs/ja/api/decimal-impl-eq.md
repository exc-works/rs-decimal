---
locale: ja
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

Rust が全等価性を要求する箇所で Decimal を使います。

```rust
impl Eq for Decimal
```

## 通常の使い方と意味

Eq はメソッドを持たないマーカートレイトです。等価性は数値で判定し、スケールや初期化状態を無視します。JavaScript オブジェクトの同一性を意味するものではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
