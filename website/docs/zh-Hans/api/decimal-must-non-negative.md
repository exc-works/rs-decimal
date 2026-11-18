---
locale: zh-Hans
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

要求数值非负，返回已初始化的副本。

```rust
fn must_non_negative (& self) -> Self
```

## 参数、返回值及常规用法

无参数；非负输入（包括零）返回 Decimal。负输入在 Rust 中以字符串载荷 panic；在 Wasm 中是 trap，不是可恢复的 DecimalError，会结束本次 playground 运行。重新运行会创建新的 Wasm 状态。这是数值断言，不是绝对值转换；改变符号请使用 abs。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
