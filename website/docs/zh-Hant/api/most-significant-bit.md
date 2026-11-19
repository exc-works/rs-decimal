---
locale: zh-Hant
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
translation_provenance: zh-Hans-terminology-reviewed
api_id: most_significant_bit
---
# most_significant_bit

查找從零起算的最高置位索引。

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## 參數、回傳值及常規用法

value: &BigInt，以 bigint 表示。回傳 Result<usize, DecimalError>/number。零和一都回傳 0，120 回傳 6。負輸入回傳 InvalidArgument。這是自由函數，不是 Decimal 方法；需要係數時應傳入 coefficient()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
