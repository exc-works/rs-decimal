---
locale: zh-Hans
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
api_id: Decimal::quo_down
---
# Decimal::quo_down

以指定精度或输入精度相除。

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。输出小数位数为两个输入中较大者，因此两个整数输入也产生整数：1 / 3 得到 0。需要小数结果请使用 quo_with_prec。零除数返回 DivideByZero。Down 向零截断，不是向负无穷。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
