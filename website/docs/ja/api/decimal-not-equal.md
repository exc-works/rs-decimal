---
locale: ja
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

2 つの小数が異なる数値を表すか調べます。

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 引数、戻り値、使用上の注意

other: &Decimal を借用し、bool/boolean を返します。equal() の反対であり、係数やスケールのスナップショットを比較するものではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
