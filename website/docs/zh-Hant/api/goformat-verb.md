---
locale: zh-Hant
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

設定 verb 格式欄位。

```rust
verb: char
```

## 參數、回傳值及常規用法

選擇單個 Unicode 純量格式動詞。預設 v 使用緊湊定點文字；e/E 使用科學記數法，q 加引號，b 輸出二進位係數及 p-scale（不是 IEEE 浮點格式）。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
