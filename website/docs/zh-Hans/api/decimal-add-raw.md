---
locale: zh-Hans
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
api_id: Decimal::add_raw
---
# Decimal::add_raw

用 i64 整数改变未缩放的系数。

```rust
fn add_raw (& self , value : i64) -> Self
```

## 参数、返回值及常规用法

value: i64，在 JavaScript 中使用 bigint。保留小数位数：在 scale 为 2 时，加减 1 改变数值 0.01，而非 1。Rust 返回新 Decimal。JavaScript 拒绝 number 及超出 i64 范围的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
