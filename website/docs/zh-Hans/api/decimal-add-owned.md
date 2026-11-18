---
locale: zh-Hans
source_revision: cd0b0af7153d96e3d2f3f7126c5b673c301639179f9c4225b3dd81e2a74678a4
api_id: Decimal::add_owned
---
# Decimal::add_owned

消耗左操作数进行精确运算。

```rust
fn add_owned (self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的已初始化 Decimal。接收者被移动，other 仍可使用。可能复用存储，但增长仍会分配。bridge 在移动前拒绝同一对象同时作为接收者和 other。进入 Rust 后，即使资源错误也会消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
