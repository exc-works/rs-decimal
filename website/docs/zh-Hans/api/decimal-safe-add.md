---
locale: zh-Hans
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
api_id: Decimal::safe_add
---
# Decimal::safe_add

精确相加，要求结果非负。

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的新 Decimal。对齐是精确的，不丢弃数字。Rust 返回 Result：仅要求结果非负，不要求每个输入非负。负结果返回 InvalidArgument，允许零结果。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
