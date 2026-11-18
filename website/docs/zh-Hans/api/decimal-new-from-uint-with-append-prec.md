---
locale: zh-Hans
source_revision: 2f8f701187893878a876b218458ba92b8464c997083c08e62c584ef14b118454
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

## 用法注意事项和边界

负值和超出范围的值属于 bridge 参数错误。负 precision 返回 `InvalidPrecision`；过大 precision 触及独立的 Playground 资源限制。请通过保留尾零的输出查看追加的小数位数。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
