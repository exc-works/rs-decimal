---
locale: zh-Hans
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

按精确方式相乘。

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal。结果小数位数为两个输入之和，保留乘积的所有数字。核心精度溢出返回 Overflow。没有舍入模式参数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
