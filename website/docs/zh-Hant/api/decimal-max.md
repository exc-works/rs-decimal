---
locale: zh-Hant
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::max
---
# Decimal::max

複製兩個數值中較大的一個。

```rust
fn max (& self , other : & Self) -> Self
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳獨立的 Decimal。相等時保留第一個運算元的小數位數及初始化狀態。Rust 中使用 Decimal::max(&left, &right) 明確選擇借用參數的固有方法；Ord::max 會取得所有權。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
