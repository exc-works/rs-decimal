---
locale: zh-Hant
source_revision: 9b1c2ac4982ef348ecf9982f858e7f552532e21dc0129fa816673741276acaf7
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.left
---
# GoFormat.left

設定 left 格式欄位。

```rust
left: bool
```

## 參數、回傳值及常規用法

在右側填充空格。左對齊優先於數字零填充。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
