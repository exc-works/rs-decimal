---
locale: zh-Hant
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

用 i64 整數改變未縮放的係數。

```rust
fn add_raw (& self , value : i64) -> Self
```

## 參數、回傳值及常規用法

value: i64，在 JavaScript 中使用 bigint。保留小數位數：在 scale 為 2 時，加減 1 改變數值 0.01，而非 1。Rust 回傳新 Decimal。JavaScript 拒絕 number 及超出 i64 範圍的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
