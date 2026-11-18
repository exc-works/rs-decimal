---
locale: zh-Hant
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
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

## 用法注意事項和邊界

瀏覽器目標是 wasm32，接受範圍始終為 -2147483648 至 2147483647，即使原生 Rust 執行在 64 位機器上。小數、NaN 和無窮大屬於參數錯誤。

瀏覽器還應用[資源限制](../resource-policy)。JavaScript 參數在呼叫 Rust 前進行類型及範圍檢查。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
