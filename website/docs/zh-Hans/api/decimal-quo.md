---
locale: zh-Hans
source_revision: 4842619da75eb5fbcf11cdce1b3ca781ba18031962688508c5f82ef39efdcf01
api_id: Decimal::quo
---
# Decimal::quo

以指定精度或输入精度相除。

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。输出小数位数为两个输入中较大者，因此两个整数输入也产生整数：1 / 3 得到 0。需要小数结果请使用 quo_with_prec。零除数返回 DivideByZero。mode 控制舍入；Unnecessary 对不精确结果返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="zh-Hans" />

## 浏览器边界

输入均被借用，成功和普通错误都不会消耗输入。返回的 Decimal 独立拥有数据，算术结果已初始化。浏览器在调用核心前检查 [资源限制](../resource-policy)：对齐、乘积中间系数、除法缩放及输出精度。保守的预检查可能拒绝最终结果很小的运算。资源错误与数值库错误分开报告；panic 会终止运行。

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
