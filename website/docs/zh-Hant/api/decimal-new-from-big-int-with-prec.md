---
locale: zh-Hant
source_revision: 7a4da4f8ddb22857191bcc95e49dde5161555ab6aaf013e72ff12ef5cff5913d
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

此方法改變係數的小數位解釋，不改變係數本身的數字。負 precision 回傳 `InvalidPrecision`。Playground 獨立限制輸入位數和 scale；`resource` 錯誤不表示數值精度損失。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
