---
locale: zh-Hant
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

從平台原生有符號整數建立已初始化的十進位數。

```rust
fn new_from_int (value : isize) -> Self
```

## 參數和回傳值

`value: isize` 對應整數 JavaScript `number`；結果 scale 為零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="zh-Hant" />

## 使用說明

瀏覽器目標是 wasm32，接受範圍始終為 -2147483648 至 2147483647，即使原生 Rust 執行在 64 位機器上。小數、NaN 和無窮大屬於參數錯誤。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
