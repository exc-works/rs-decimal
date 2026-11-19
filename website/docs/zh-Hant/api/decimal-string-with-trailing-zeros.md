---
locale: zh-Hant
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

格式化為定點十進位文字。

```rust
fn string_with_trailing_zeros (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string，不使用科學記數法。保留所有儲存的小數位，包括末尾零。使用小數點，不分組。負零沒有單獨的表示。預設零輸出 0，不因此被初始化；格式化僅借用輸入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
