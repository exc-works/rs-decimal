---
locale: zh-Hant
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
translation_provenance: zh-Hans-terminology-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

設定 precision 格式欄位。

```rust
precision: Option < usize >
```

## 參數、回傳值及常規用法

精度含義隨動詞變化：f/F 為小數位，e/E 為首位之後的位數，g/G 為有效位數（零按一處理），d 為最少整數位。None/null 使用動詞預設規則；f 保留 scale，e 使用六位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="zh-Hant" />

## 相關方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
