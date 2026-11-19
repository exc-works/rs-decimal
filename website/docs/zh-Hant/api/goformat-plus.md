---
locale: zh-Hant
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

設定 plus 格式欄位。

```rust
plus: bool
```

## 參數、回傳值及常規用法

對非負數字輸出顯示 +，優先於 space。適用於數字動詞，不適用於 v/s/q；負數保留 -。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
