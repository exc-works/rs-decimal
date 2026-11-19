---
locale: zh-Hant
source_revision: 742bab219fea4dd88e3233aac7a28750ce01392539e76e100af497e171dcc4c9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::big_rat
---
# Decimal::big_rat

讀取精確約分後的有理數值。

```rust
fn big_rat (& self) -> BigRational
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 BigRational。JavaScript 回傳不可變的 {numerator: bigint, denominator: bigint} 快照，正規化後分母為正。Decimal 的精度和末尾零不會作為有理數中繼資料保留；零變為 0/1。輸入僅被借用，保持不變。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-big-rat" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
