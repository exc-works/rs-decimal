---
locale: zh-Hans
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

消耗 Decimal，同时改变小数位数。

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

prec: isize；mode: RoundingMode。与 rescale 一样，舍入或追加零，并始终初始化结果。即使 Rust 返回 InvalidPrecision、RoundUnnecessary 或浏览器资源错误，接收者也已移动。JS 类型或范围不合法会在进入 Rust 前被拒绝，不消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
