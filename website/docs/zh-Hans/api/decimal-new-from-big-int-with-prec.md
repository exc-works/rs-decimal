---
locale: zh-Hans
source_revision: 7a4da4f8ddb22857191bcc95e49dde5161555ab6aaf013e72ff12ef5cff5913d
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

复制任意精度系数并指定小数位数，使大整数输入保持精确。

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: &BigInt` 对应 `bigint`；`prec: isize` 对应整数 `number`。结果为 `value × 10^-prec`，Rust 输入采用借用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="zh-Hans" />

## 使用说明

此方法改变系数的小数位解释，不改变系数本身的数字。负 precision 返回 `InvalidPrecision`。Playground 独立限制输入位数和 scale；`resource` 错误不表示数值精度损失。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
