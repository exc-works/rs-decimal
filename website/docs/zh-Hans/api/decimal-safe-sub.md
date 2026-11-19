---
locale: zh-Hans
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

精确相减，要求结果非负。

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的新 Decimal。对齐是精确的，不丢弃数字。Rust 返回 Result：仅要求结果非负，不要求每个输入非负。负结果返回 InvalidArgument，允许零结果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
