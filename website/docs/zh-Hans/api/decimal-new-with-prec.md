---
locale: zh-Hans
source_revision: 78d4bcb73439eeefbcaa373b59f6572230d9fd809bde6e3821566e584e60e547
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

把有符号整数当作指定小数位数的系数。例如系数 12、precision 为 2 表示 0.12。

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: i64` 对应 `bigint`；`prec: isize` 对应整数 `number`，结果为 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="zh-Hans" />

## 用法注意事项和边界

precision 表示小数位数，不是有效数字位数。它不会在保持整数值不变的前提下追加零；需要 12.00 时使用 `new_with_append_prec`。负 precision 返回 `InvalidPrecision`，零系数也保留指定 scale。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
