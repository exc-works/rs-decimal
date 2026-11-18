---
locale: zh-Hant
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sign
---
# Decimal::sign

以 -1、0、1 讀取符號。

```rust
fn sign (& self) -> i32
```

## 參數、回傳值及注意事項

回傳 i32/number。已初始化和未初始化的零都回傳 0，不保留單獨的負零符號。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
