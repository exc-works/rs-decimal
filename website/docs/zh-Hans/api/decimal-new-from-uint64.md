---
locale: zh-Hans
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
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

## 用法注意事项和边界

负值和超过 u64::MAX 的值会在进入 Rust 前被拒绝。负 precision 返回库错误 `InvalidPrecision`。JavaScript number 无法安全承载完整的 u64 范围。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
