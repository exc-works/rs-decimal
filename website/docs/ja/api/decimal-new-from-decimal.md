---
locale: ja
source_revision: 3e32f687004e72286fa606992f7977ccbc210f3d36756c6006ab536cd5091f5a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

保存された表現と初期化状態を含め、既存の Decimal をコピーします。

```rust
fn new_from_decimal (value : & Self) -> Self
```

## 引数と戻り値

`value: &Decimal` を借用します。返された Decimal は独立しており、入力も引き続き使用できます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="ja" />

## 使い方の補足

`Decimal::default()` のコピーでは未初期化状態を保持し、初期化済みゼロにはしません。JavaScript では同じ実行に属する使用可能な Decimal を渡してください。このコンストラクターは入力を消費しません。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
