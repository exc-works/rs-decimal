---
locale: zh-Hant
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::value"
---
# Decimal::value

取得規範的 SQL 值。

```rust
fn value (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。即使 Decimal 未初始化也回傳非 NULL 的 "0"；需要 SQL NULL 時使用 NullDecimal。移除小數末尾零，但不改變 scale 或初始化狀態。回傳文字是純量值，不是帶引號的 SQL 查詢片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
