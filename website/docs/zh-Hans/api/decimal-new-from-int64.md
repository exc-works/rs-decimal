---
locale: zh-Hans
source_revision: fae9d1b20487be5ea60841b603ab060b40d5b639bd9efc686e9fd8f0f555bd79
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

从有符号 64 位系数和小数位数创建 Decimal，数值及状态行为与 `new_with_prec` 相同。

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: i64` 对应 `bigint`，`prec: isize` 对应整数 `number`；结果为 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="zh-Hans" />

## 使用说明

value 是系数，不是已经带有小数位的金额。负 precision 返回 `InvalidPrecision`；超出 i64 范围的值属于 JavaScript 参数错误。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
