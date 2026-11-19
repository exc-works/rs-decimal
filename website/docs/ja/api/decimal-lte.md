---
locale: ja
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::lte
---
# Decimal::lte

この値が他の値以下か調べます。

```rust
fn lte (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。等価性は数値で判定し、スケールや初期化状態には依存しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
