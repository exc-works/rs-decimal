---
locale: zh-Hans
source_revision: c4fc0f977bb46b55b53465df0296b049aa252fbfa98d47634f13b46fd3108bd0
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

从无符号 64 位系数创建 Decimal，适合非负整数来源且小数位数单独提供的场景。

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 参数和返回值

`value: u64` 对应非负 `bigint`；`prec: isize` 对应整数 `number`，结果为 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="zh-Hans" />

## 使用说明

负值和超过 u64::MAX 的值会在进入 Rust 前被拒绝。负 precision 返回库错误 `InvalidPrecision`。JavaScript number 无法安全承载完整的 u64 范围。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
