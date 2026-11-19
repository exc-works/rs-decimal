---
locale: zh-Hant
source_revision: c983f6ed089705857a773a0bcfbea0bf32771eb2c76c31e24f0160ce317c8980
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

用 i64 整數改變未縮放的係數。

```rust
fn add_raw (& self , value : i64) -> Self
```

## 參數、回傳值及常規用法

value: i64，在 bridge 中使用 bigint。保留小數位數：在 scale 為 2 時，加減 1 改變數值 0.01，而非 1。Rust 回傳新 Decimal。bridge 拒絕 number 及超出 i64 範圍的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
