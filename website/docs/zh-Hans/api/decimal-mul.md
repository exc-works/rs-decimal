---
locale: zh-Hans
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
api_id: Decimal::mul
---
# Decimal::mul

按明确的输出精度相乘。

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal。结果小数位数为两个输入中较大者，而非二者之和。保留乘积全部数字请使用 mul_exact。显式 mode 控制舍入；Unnecessary 在必须丢弃非零数字时返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
