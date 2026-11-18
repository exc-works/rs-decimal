---
locale: zh-Hant
source_revision: b5a3d9a8d6370fd602bde442c0194ed8033b601f5d907d38c4cf9f51317197c6
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

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
