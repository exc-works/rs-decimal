---
locale: ja
source_revision: 3ced5d99beade676205267c80e93f90c49f974486393990035a12b18f4061c4a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl PartialEq
---
# Decimal::impl PartialEq

Rust の演算子または PartialEq で数値の等価性を比較します。

```rust
impl PartialEq for Decimal
```

## 通常の使い方と意味

eq は両オペランドを借用し、bool を返します。12.00 == 12.0 です。JavaScript では equal() を使います。=== はラッパーの同一性を調べます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
