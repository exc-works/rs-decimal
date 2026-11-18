---
locale: zh-Hant
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::precision
---
# Decimal::precision

讀取數值儲存的小數位數。

```rust
fn precision (& self) -> isize
```

## 參數、回傳值及注意事項

回傳 isize，在瀏覽器中為整數 number。這是小數位數，不是有效位數位數；末尾零也計入其中。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
