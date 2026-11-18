---
locale: ja
source_revision: 37b6ebebec69abdf2809e71eb7e3cad7353a792a069c7217103a51f2eea16c7f
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

## 使用上の注意と制約

`Decimal::default()` のコピーでは未初期化状態を保持し、初期化済みゼロにはしません。JavaScript では同じ実行に属する使用可能な Decimal を渡してください。このコンストラクターは入力を消費しません。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
