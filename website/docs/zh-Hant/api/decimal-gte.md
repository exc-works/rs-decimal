---
locale: zh-Hant
source_revision: 03df94c2d70888cf880122deea0e6ec382870fa1e10b3d6cb2d26b3fc0fb348a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::gte
---
# Decimal::gte

判斷目前數值是否大於或等於另一個數值。

```rust
fn gte (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。相等按數值判斷，與小數位數及初始化狀態無關。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
