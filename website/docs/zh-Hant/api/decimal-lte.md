---
locale: zh-Hant
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::lte
---
# Decimal::lte

判斷目前數值是否小於或等於另一個數值。

```rust
fn lte (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。相等按數值判斷，與小數位數及初始化狀態無關。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
