---
locale: zh-Hans
source_revision: 5a350c30c073c772c4921158a8a0d829d4480ae375ec35f7532a213f3f45e557
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

保持有符号整数的数值不变，同时增加存储的小数位数，适合把整数金额表示为 12.00。

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: i64` 对应 `bigint`；`prec: isize` 对应整数 `number`。存储系数变为 `value × 10^prec`，scale 为 `prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="zh-Hans" />

## 使用说明

`string()` 输出 12，`string_with_trailing_zeros()` 输出 12.00。与 `new_with_prec` 不同，数值不会变成 0.12。负 precision 返回 `InvalidPrecision`；过大的请求在分配十次幂之前被拒绝。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
