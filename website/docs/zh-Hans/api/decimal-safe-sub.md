---
locale: zh-Hans
source_revision: 8969e894369c10d2e27ead76f07916d45a77105ffad604c0d4f46fcf59b84751
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

精确相减，要求结果非负。

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的新 Decimal。对齐是精确的，不丢弃数字。Rust 返回 Result：仅要求结果非负，不要求每个输入非负。负结果返回 InvalidArgument，允许零结果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
