---
locale: zh-Hant
source_revision: ecdf018f489484305dd62f934b29d0a6759b6939d7abb56d9717dd4b72e36899
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

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
