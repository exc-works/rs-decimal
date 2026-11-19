---
locale: zh-Hant
source_revision: e9defed137146374a74cf0b0a14760988cf4cd8ad97babd7fd11a73ccc7037b3
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

檢查數值的小數部分。

```rust
fn has_fraction (& self) -> bool
```

## 參數、回傳值及常規用法

無參數，回傳 bool/boolean。檢查數值而非 scale 是否為零：12.00 仍是整數。預設零也是整數，沒有小數部分。查詢不會初始化或修改輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
