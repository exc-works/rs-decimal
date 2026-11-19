---
locale: zh-Hant
source_revision: 31042d691902e2dc4bdadd92d4688638c315fb08be40c79d11b031b854afc6bc
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

用 i64 整數改變未縮放的係數。

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 參數、回傳值及常規用法

value: i64，在 bridge 中使用 bigint。保留小數位數：在 scale 為 2 時，加減 1 改變數值 0.01，而非 1。Rust 回傳新 Decimal。bridge 拒絕 number 及超出 i64 範圍的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
