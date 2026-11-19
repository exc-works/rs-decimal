---
locale: ja
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::equal
---
# Decimal::equal

2 つの小数が同じ数値を表すか調べます。

```rust
fn equal (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。スケールと初期化状態は無視します。JavaScript の === はラッパーの同一性を比較します。Rust の数値としての等価性には equal() を使ってください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
