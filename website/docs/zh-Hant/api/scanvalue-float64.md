---
locale: zh-Hant
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

選擇 Float64 SQL 輸入變體。

```rust
Float64 (f64)
```

## 參數、回傳值及常規用法

傳遞 binary64。有限值使用核心最短十進位轉換；NaN 和無窮回傳 Unmarshal。已捨入的 JS number 無法恢復原始數字。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
