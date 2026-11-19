---
locale: zh-Hant
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.space
---
# GoFormat.space

設定 space 格式欄位。

```rust
space: bool
```

## 參數、回傳值及常規用法

plus 為 false 時，為非負數字輸出添加前導空格。負數保留符號。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
