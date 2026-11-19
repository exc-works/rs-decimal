---
locale: zh-Hans
source_revision: 0b7e4ede82a7865b24f3ea40debe791505bf9ac255c8e2f8790aa1271d0fdf3a
api_id: Decimal::int_part
---
# Decimal::int_part

向零截断，提取整数部分。

```rust
fn int_part (& self) -> BigInt
```

## 参数、返回值及常规用法

无参数，Rust 返回 BigInt，JavaScript 返回 bigint，不经过浮点转换。-12.75 得到 -12，而非 floor(-12.75) 的 -13。默认零得到零。示例将整数序列化为字符串，以在 JSON 中保留任意精度。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
