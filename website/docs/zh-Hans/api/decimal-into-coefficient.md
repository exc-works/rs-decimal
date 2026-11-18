---
locale: zh-Hans
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
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

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
