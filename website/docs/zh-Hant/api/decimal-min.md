---
locale: zh-Hant
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::min
---
# Decimal::min

複製兩個數值中較小的一個。

```rust
fn min (& self , other : & Self) -> Self
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳獨立的 Decimal。相等時保留第一個運算元的小數位數及初始化狀態。Rust 中使用 Decimal::min(&left, &right)，避免選中會取得所有權的 Ord::min trait 方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
