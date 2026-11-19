---
locale: zh-Hant
source_revision: 89b834d2c11952f4f6bb0f4503b5e891bf052db7e48a72abe6c367e79b9b8c68
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

複製任意精度係數並指定小數位數，使大整數輸入保持精確。

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: &BigInt` 對應 `bigint`；`prec: isize` 對應整數 `number`。結果為 `value × 10^-prec`，Rust 輸入採用借用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="zh-Hant" />

## 使用說明

此方法改變係數的小數位解釋，不改變係數本身的數字。負 precision 回傳 `InvalidPrecision`。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
