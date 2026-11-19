---
locale: zh-Hant
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

移動出未縮放的整數係數。

```rust
fn into_coefficient (self) -> BigInt
```

## 參數、回傳值及常規用法

無參數；Rust 回傳 BigInt，JS 回傳 bigint。丟棄小數位數和初始化狀態，因此 1.2300 得到 12300。預設零得到 0。Decimal 被消耗，不能再次讀取；bridge 中需要不消耗的快照時使用 coefficient。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
