---
locale: zh-Hans
source_revision: b1a9d67b6db859834f3ef9cd2d0fefcd4239bbf10472aedba8e0a86a95b51eeb
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

保持无符号整数的数值不变，同时追加存储的小数位数。

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: u64` 对应非负 `bigint`；`prec: isize` 对应整数 `number`，结果保持原整数值并采用 scale `prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="zh-Hans" />

## 使用说明

负值和超出范围的值属于 JavaScript 参数错误。负 precision 返回 `InvalidPrecision`。请通过保留尾零的输出查看追加的小数位数。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
