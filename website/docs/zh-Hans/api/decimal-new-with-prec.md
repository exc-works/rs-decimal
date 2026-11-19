---
locale: zh-Hans
source_revision: 2cba3a645b36c53756ae013b43e89956dee5ee0f736cbeccecbeb94e8966c7b4
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

## 使用说明

precision 表示小数位数，不是有效数字位数。它不会在保持整数值不变的前提下追加零；需要 12.00 时使用 `new_with_append_prec`。负 precision 返回 `InvalidPrecision`，零系数也保留指定 scale。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
