---
locale: zh-Hant
source_revision: f893b87d46c7c5082ca48cb1eae39734e63ca91a57f2f944773172a9baf78fca
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

設定 zero 格式欄位。

```rust
zero: bool
```

## 參數、回傳值及常規用法

在符號之後用零填充數字左側。left 為 true 或 d 有顯式 precision 時忽略。預設為 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
