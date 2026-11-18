---
locale: zh-Hans
source_revision: 8c45e8f4230346a7c6fcd3cbb920d429b7a02acbcd8eb534c1f096fa8233828d
api_id: Decimal::add
---
# Decimal::add

精确相加。

```rust
fn add (& self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的新 Decimal。对齐是精确的，不丢弃数字。Rust 方法本身不返回错误；浏览器仍可能因分配限制返回资源错误。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
