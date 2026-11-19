---
locale: zh-Hant
source_revision: 88974eef4784bf8f1c8b6ec4ce25d3380a2169d449467ecdb4cee7997d0acdd8
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for GoFormat
```

## 參數、回傳值及常規用法

GoFormat 是小型 Copy 欄位結構體。Clone/Copy 保留選項；Debug 輸出欄位。Default 選擇 v，width/precision 為 None，所有旗標為 false。此 Rust 語法在原生環境測試；JavaScript 物件使用普通 JS 欄位物件語義及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
