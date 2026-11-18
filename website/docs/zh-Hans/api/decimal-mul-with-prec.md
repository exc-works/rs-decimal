---
locale: zh-Hans
source_revision: ce941de7d4ef355a538d2c2c33a609ab4eeb3b4645eb801e81a24755fcf0ca5a
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

按明确的输出精度相乘。

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal。结果保留 prec 个小数位；prec 必须非负，否则返回 InvalidPrecision。显式 mode 控制舍入；Unnecessary 在必须丢弃非零数字时返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
