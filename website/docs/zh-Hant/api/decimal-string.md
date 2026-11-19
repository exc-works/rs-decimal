---
locale: zh-Hant
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::string
---
# Decimal::string

格式化為定點十進位文字。

```rust
fn string (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string，不使用科學記數法。移除多餘的小數末尾零，保留整數末尾零。使用小數點，不分組。負零沒有單獨的表示。預設零輸出 0，不因此被初始化；格式化僅借用輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
