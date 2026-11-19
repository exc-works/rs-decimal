---
locale: zh-Hans
source_revision: 784a138fd9dfff6c4bbabb3b837f6e63c10c904c5578943fe043993afcf99a55
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

移动出未缩放的整数系数。

```rust
fn into_coefficient (self) -> BigInt
```

## 参数、返回值及常规用法

无参数；Rust 返回 BigInt，JS 返回 bigint。丢弃小数位数和初始化状态，因此 1.2300 得到 12300。默认零得到 0。Decimal 被消耗，不能再次读取；bridge 中需要不消耗的快照时使用 coefficient。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
