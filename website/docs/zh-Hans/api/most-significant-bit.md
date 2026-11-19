---
locale: zh-Hans
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
api_id: most_significant_bit
---
# most_significant_bit

查找从零起算的最高置位索引。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 参数、返回值及常规用法

value: &BigInt，以 bigint 表示。返回 Result<usize, DecimalError>/number。零和一都返回 0，120 返回 6。负输入返回 InvalidArgument。这是自由函数，不是 Decimal 方法；需要系数时应传入 coefficient()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
