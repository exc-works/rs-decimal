---
locale: zh-Hans
source_revision: 3e67428ba1ea89053920f2f5796e826dd297f81c12e673f6f4e20f40216c28ff
api_id: Decimal::big_rat
---
# Decimal::big_rat

读取精确约分后的有理数值。

```rust
fn big_rat (& self) -> BigRational
```

## 参数、返回值及常规用法

无参数，Rust 返回 BigRational。bridge 返回不可变的 {numerator: bigint, denominator: bigint} 快照，规范化后分母为正。Decimal 的精度和末尾零不会作为有理数元数据保留；零变为 0/1。输入仅被借用，保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
