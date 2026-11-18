---
locale: zh-Hans
source_revision: 96feaba7858db9f7e7efab2f2187c38abada064aba6bc3c66092f84f0b94b74f
api_id: Decimal::quo_down
---
# Decimal::quo_down

以指定精度或输入精度相除。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。输出小数位数为两个输入中较大者，因此两个整数输入也产生整数：1 / 3 得到 0。需要小数结果请使用 quo_with_prec。零除数返回 DivideByZero。Down 向零截断，不是向负无穷。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
