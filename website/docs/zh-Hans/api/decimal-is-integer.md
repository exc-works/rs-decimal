---
locale: zh-Hans
source_revision: 46411f1a8f7889f0ec3654fae25da003c49b8d268105e0431ed703cb61c031fe
api_id: Decimal::is_integer
---
# Decimal::is_integer

检查数值的小数部分。

```rust
fn is_integer (& self) -> bool
```

## 参数、返回值及常规用法

无参数，返回 bool/boolean。检查数值而非 scale 是否为零：12.00 仍是整数。默认零也是整数，没有小数部分。查询不会初始化或修改输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
