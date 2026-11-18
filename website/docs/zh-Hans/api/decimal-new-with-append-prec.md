---
locale: zh-Hans
source_revision: 7e14f36bf17f5e06d930d4aefd01137eff4589a325eb7eee0237e89f396a3ba8
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

## 用法注意事项和边界

`string()` 输出 12，`string_with_trailing_zeros()` 输出 12.00。与 `new_with_prec` 不同，数值不会变成 0.12。负 precision 返回 `InvalidPrecision`；过大的请求在分配十次幂之前被拒绝。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
