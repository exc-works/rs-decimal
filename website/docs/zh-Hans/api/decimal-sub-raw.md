---
locale: zh-Hans
source_revision: 3fdc16376a6b5df85a2ce7cc24f6688fcdf41771e793df75d13c0d1b90a448b1
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

用 i64 整数改变未缩放的系数。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 参数、返回值及常规用法

value: i64，在 JavaScript 中使用 bigint。保留小数位数：在 scale 为 2 时，加减 1 改变数值 0.01，而非 1。Rust 返回新 Decimal。JavaScript 拒绝 number 及超出 i64 范围的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
