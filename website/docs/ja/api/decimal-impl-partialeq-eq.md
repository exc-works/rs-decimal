---
locale: ja
source_revision: 1d252fa81f50f2414d333721b6e7df59a5331a4689705908696e9896f6fb4dbf
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::impl PartialEq::eq
---
# Decimal::impl PartialEq::eq

Rust の演算子または PartialEq で数値の等価性を比較します。

```rust
fn eq (& self , other : & Self) -> bool
```

## 通常の使い方と意味

eq は両オペランドを借用し、bool を返します。12.00 == 12.0 です。JavaScript では equal() を使います。=== はラッパーの同一性を調べます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialeq-eq" locale="ja" />

この Rust 構文の例はネイティブテストで実行します。ブラウザーでのトレイト対応を意味しません。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
