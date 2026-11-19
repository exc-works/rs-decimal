---
locale: zh-Hant
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Text"
---
# ScanValue::Text

選擇 Text SQL 輸入變體。

```rust
Text (& 'a str)
```

## 參數、回傳值及常規用法

借用精確小數文字。移除一對匹配的首尾雙引號，但不按 JSON 解碼跳脫。解析後保留非零值的末尾 scale。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
