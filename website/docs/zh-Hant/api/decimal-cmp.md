---
locale: zh-Hant
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

用三態結果比較兩個數值。

```rust
fn cmp (& self , other : & Self) -> i32
```

## 參數、回傳值及注意事項

借用 other: &Decimal（JavaScript 中為 Decimal）。回傳 i32/number：-1、0 或 1。小數位數及初始化狀態不同不影響數值相等。這是固有方法，不是回傳 Ordering 的 Ord::cmp。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
