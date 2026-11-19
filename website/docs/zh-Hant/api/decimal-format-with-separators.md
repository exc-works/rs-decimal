---
locale: zh-Hant
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

對整數位分組，並選擇小數分隔符。

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 參數、回傳值及常規用法

thousands: char；decimal: char。回傳 String/string，整數位每三位一組，保留所有儲存的小數位。每個 JS 參數必須恰好是一個 Unicode 純量，可使用補充平面字元；孤立代理項和多字元字串被拒絕。NUL thousands 禁用分組，NUL decimal 使用小數點。分隔符是字面字元，不是語言區域名稱。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
