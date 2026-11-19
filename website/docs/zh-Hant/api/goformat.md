---
locale: zh-Hant
source_revision: e057a9c108e2aa51ba03c94655793b13aece7049e3b5085cd912fedc0ed1d203
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat
---
# GoFormat

使用欄位物件描述一次格式化操作。

```rust
pub struct GoFormat
```

## 參數、回傳值及常規用法

欄位對應 Rust 結構體：verb、width、precision、left、plus、space、zero、alternate。JavaScript 的 GoFormat.default() 從 Rust 讀取預設值，回傳新的可變物件；修改它不會設定全域格式。width/precision 使用 null 表示 None，非負 wasm32 整數 number 表示 Some。Rust trait 仍屬於原生語法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
