---
locale: ja
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::lt
---
# Decimal::lt

この値が他の値より小さいか調べます。

```rust
fn lt (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。保存されたスケールが異なっても、数値が等しければ false です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
