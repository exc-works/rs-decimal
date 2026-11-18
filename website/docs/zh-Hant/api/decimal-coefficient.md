---
locale: zh-Hant
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

查看精確的未縮放整數係數。

```rust
fn coefficient (& self) -> & BigInt
```

## 參數、回傳值及注意事項

Rust 借用 &BigInt；bridge 回傳不可變的 bigint 快照，並非即時參照。係數包含末尾零，不是小數的整數部分。範例把 bigint 轉成文字，以便 JSON 保留全部數字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="zh-Hant" />

## 錯誤及邊界

這些方法借用輸入，不會消耗或改變輸入。程式庫操作本身不回傳 Result。bridge 拒絕已釋放、已移動或來自其他執行實例的物件，並應用 [資源限制](../resource-policy)。範例用 `value` 欄位展示標量結果；Decimal 結果展示完整快照。

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
