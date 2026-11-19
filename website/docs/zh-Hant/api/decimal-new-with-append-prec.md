---
locale: zh-Hant
source_revision: 845ec62ddce327ca5b797a7a47302c991bc3fbeb2f329134a32bdc0e5d88f53e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

保持有符號整數的數值不變，同時增加儲存的小數位數，適合把整數金額表示為 12.00。

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: i64` 對應 `bigint`；`prec: isize` 對應整數 `number`。儲存係數變為 `value × 10^prec`，scale 為 `prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="zh-Hant" />

## 使用說明

`string()` 輸出 12，`string_with_trailing_zeros()` 輸出 12.00。與 `new_with_prec` 不同，數值不會變成 0.12。負 precision 回傳 `InvalidPrecision`。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
