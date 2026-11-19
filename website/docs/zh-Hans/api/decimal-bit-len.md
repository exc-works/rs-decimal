---
locale: zh-Hans
source_revision: 3cc2ce52c4d0ed0d111f2e4df3eea8f2b90cb54656fa97590458dae94eb7b967
api_id: Decimal::bit_len
---
# Decimal::bit_len

统计未缩放系数绝对值的二进制位数。

```rust
fn bit_len (& self) -> usize
```

## 参数、返回值及常规用法

无参数，Rust 返回 usize，bridge 返回 number。不应用 scale，也不计符号位：-1.20 的系数为 -120，位数为 7。零（包括默认零）返回 0。这不是最高置位的索引。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
