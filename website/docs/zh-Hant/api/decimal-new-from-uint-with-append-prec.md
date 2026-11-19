---
locale: zh-Hant
source_revision: b1a9d67b6db859834f3ef9cd2d0fefcd4239bbf10472aedba8e0a86a95b51eeb
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

保持無符號整數的數值不變，同時追加儲存的小數位數。

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: u64` 對應非負 `bigint`；`prec: isize` 對應整數 `number`，結果保持原整數值並採用 scale `prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="zh-Hant" />

## 使用說明

負值和超出範圍的值屬於 JavaScript 參數錯誤。負 precision 回傳 `InvalidPrecision`。請透過保留尾零的輸出查看追加的小數位數。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
