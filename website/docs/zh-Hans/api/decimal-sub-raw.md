---
locale: zh-Hans
source_revision: 4d452c3c2c6d2bed5b36a82866f10106b5603e80ad3916a0ac4e55ced6bfc3f3
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

用 i64 整数改变未缩放的系数。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 参数、返回值及常规用法

value: i64，在 bridge 中使用 bigint。保留小数位数：在 scale 为 2 时，加减 1 改变数值 0.01，而非 1。Rust 返回新 Decimal。bridge 拒绝 number 及超出 i64 范围的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
