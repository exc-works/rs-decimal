---
locale: zh-Hant
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::neg
---
# Decimal::neg

建立符號相反的獨立 Decimal。

```rust
fn neg (& self) -> Self
```

## 參數、回傳值及注意事項

回傳 Decimal，保留小數位數及末尾零。零在數值上仍為零；即使輸入為預設值，結果也會初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
