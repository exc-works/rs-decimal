---
locale: zh-Hans
source_revision: e9defed137146374a74cf0b0a14760988cf4cd8ad97babd7fd11a73ccc7037b3
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

检查数值的小数部分。

```rust
fn has_fraction (& self) -> bool
```

## 参数、返回值及常规用法

无参数，返回 bool/boolean。检查数值而非 scale 是否为零：12.00 仍是整数。默认零也是整数，没有小数部分。查询不会初始化或修改输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
