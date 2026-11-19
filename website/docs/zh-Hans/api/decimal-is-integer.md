---
locale: zh-Hans
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
api_id: Decimal::is_integer
---
# Decimal::is_integer

检查数值是否没有小数部分（是否为整数）。

```rust
fn is_integer (& self) -> bool
```

## 参数、返回值及常规用法

无参数，返回 bool/boolean。检查数值而非 scale 是否为零：12.00 仍是整数。默认零也是整数，没有小数部分。查询不会初始化或修改输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
