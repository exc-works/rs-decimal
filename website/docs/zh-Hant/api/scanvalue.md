---
locale: zh-Hant
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue"
---
# ScanValue

選擇與驅動無關的 SQL 輸入類型。

```rust
pub enum ScanValue< 'a >
```

## 參數、回傳值及常規用法

此列舉區分 NULL、f32、f64、i64、文字、文字位元組和不支援的值。呼叫 Decimal.scan 或 NullDecimal.scan 前明確選擇變體。Rust 借用的文字/位元組在呼叫期間有效即可；瀏覽器 Bytes 保留呼叫者陣列，並觀察 scan 前的修改。不連接資料庫服務。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
