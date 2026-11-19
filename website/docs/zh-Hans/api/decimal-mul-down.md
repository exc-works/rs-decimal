---
locale: zh-Hans
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
api_id: Decimal::mul_down
---
# Decimal::mul_down

按明确的输出精度相乘。

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal。结果小数位数为两个输入中较大者，而非二者之和。保留乘积全部数字请使用 mul_exact。Down 向零截断，包括负结果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
