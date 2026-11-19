---
locale: zh-Hant
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::not_equal
---
# Decimal::not_equal

判斷兩個 Decimal 的數值是否不同。

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。這是 equal() 的反向判斷，不是對係數及小數位數快照的比較。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
