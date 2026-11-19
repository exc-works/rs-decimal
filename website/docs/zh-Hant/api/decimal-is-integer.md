---
locale: zh-Hant
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

檢查數值是否沒有小數部分（是否為整數）。

```rust
fn is_integer (& self) -> bool
```

## 參數、回傳值及常規用法

無參數，回傳 bool/boolean。檢查數值而非 scale 是否為零：12.00 仍是整數。預設零也是整數，沒有小數部分。查詢不會初始化或修改輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
