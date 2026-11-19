---
locale: zh-Hant
source_revision: 9ec084e5d70a80123e6b5391a460c2d09074b7e750d373283106219a5cde28ed
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

從有符號 64 位係數和小數位數建立 Decimal，數值及狀態行為與 `new_with_prec` 相同。

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: i64` 對應 `bigint`，`prec: isize` 對應整數 `number`；結果為 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="zh-Hant" />

## 使用說明

value 是係數，不是已經帶有小數位的金額。負 precision 回傳 `InvalidPrecision`；超出 i64 範圍的值屬於 bridge 參數錯誤。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
