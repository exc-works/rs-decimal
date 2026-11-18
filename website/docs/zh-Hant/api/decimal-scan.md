---
locale: zh-Hant
source_revision: 805fc62e495d042f613bdfd51817251dea41dcde63bd630737a6a7dd4ec14e8a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::scan"
---
# Decimal::scan

讀取與資料庫驅動無關的 SQL 純量。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

value: ScanValue；修改接收者，回傳 `Result<(), DecimalError>`/undefined。明確選擇變體：Int64 使用精確 bigint，Float32 收窄為 binary32，Text/Bytes 保留原始小數數字。SQL 文字可帶一對首尾雙引號，只移除引號而不進行 JSON 反跳脫。NULL 將接收者重置為未初始化零。不支援的類型和非有限浮點數回傳 Unmarshal，其他解碼錯誤保留底層類別。所有錯誤均保留接收者狀態。此適配器不需要資料庫連線。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
