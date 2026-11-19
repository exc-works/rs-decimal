---
locale: zh-Hans
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
api_id: Decimal::bit_len
---
# Decimal::bit_len

统计未缩放系数绝对值的二进制位数。

```rust
fn bit_len (& self) -> usize
```

## 参数、返回值及常规用法

无参数，Rust 返回 usize，JavaScript 返回 number。不应用 scale，也不计符号位：-1.20 的系数为 -120，位数为 7。零（包括默认零）返回 0。这不是最高置位的索引。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
