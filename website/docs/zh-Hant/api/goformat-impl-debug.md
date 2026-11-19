---
locale: zh-Hant
source_revision: 31f0a432b373e9f77b1c47a8c72a2cfedcd5df17aa590c851611115cc0270629
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat::impl Debug
---
# GoFormat::impl Debug

使用 Rust Debug 實現。

```rust
impl Debug for GoFormat
```

## 參數、回傳值及常規用法

GoFormat 是小型 Copy 欄位結構體。Clone/Copy 保留選項；Debug 輸出欄位。Default 選擇 v，width/precision 為 None，所有旗標為 false。此 Rust 語法在原生環境測試；JavaScript 物件使用普通 JS 欄位物件語義及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-debug" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
