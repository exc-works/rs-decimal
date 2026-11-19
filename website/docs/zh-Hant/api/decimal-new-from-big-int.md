---
locale: zh-Hant
source_revision: 41da382d7882f55308ed631f052fb5ac78093f335752392d35d023f744890035
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

把任意精度整數複製為已初始化、scale 為零的 Decimal，不丟失數字。

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## 參數和回傳值

Rust 借用 `&BigInt`；JavaScript 傳入不可變的 `bigint`。回傳值的係數獨立於來源。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="zh-Hant" />

## 使用說明

不要先用 JavaScript number 丟失精度後再轉 bigint；應直接使用以 `n` 結尾的整數字面量或精確整數文字。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
