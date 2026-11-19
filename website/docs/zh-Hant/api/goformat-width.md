---
locale: zh-Hant
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.width
---
# GoFormat.width

設定 width 格式欄位。

```rust
width: Option < usize >
```

## 參數、回傳值及常規用法

最小寬度包含符號和小數點；透過填充補足，不截斷更長文字。None/null 禁用最小寬度。核心按輸出位元組數計算。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
