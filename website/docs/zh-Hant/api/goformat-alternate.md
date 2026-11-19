---
locale: zh-Hant
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

設定 alternate 格式欄位。

```rust
alternate: bool
```

## 參數、回傳值及常規用法

對 f/F/e/E 保留小數點，對 g/G 保留一般格式的末尾零。不是改變數字進位。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
