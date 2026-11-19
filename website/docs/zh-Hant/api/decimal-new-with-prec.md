---
locale: zh-Hant
source_revision: 2cba3a645b36c53756ae013b43e89956dee5ee0f736cbeccecbeb94e8966c7b4
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

把有符號整數當作指定小數位數的係數。例如係數 12、precision 為 2 表示 0.12。

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: i64` 對應 `bigint`；`prec: isize` 對應整數 `number`，結果為 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="zh-Hant" />

## 使用說明

precision 表示小數位數，不是有效位數位數。它不會在保持整數值不變的前提下追加零；需要 12.00 時使用 `new_with_append_prec`。負 precision 回傳 `InvalidPrecision`，零係數也保留指定 scale。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
