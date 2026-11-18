---
locale: zh-Hant
source_revision: 79298c474720d5ca067e6e36ecdefe721611687b58bab5fff1d35e1889208bcd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

取得規範的 SQL 值。

```rust
fn value (& self) -> Option < String >
```

## 參數、回傳值及常規用法

無參數，回傳 `Option<String>`/string | null。無效 nullable 回傳 None/null，與隱藏內部值無關。有效值回傳規範小數文字，內部零未初始化時也回傳 "0"。移除小數末尾零，但不改變 scale 或初始化狀態。回傳文字是純量值，不是帶引號的 SQL 查詢片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
