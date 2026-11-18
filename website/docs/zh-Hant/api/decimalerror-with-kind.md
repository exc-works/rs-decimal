---
locale: zh-Hant
source_revision: f5327cbdbb40212ac4046096b90742ab088011d23930faef1e12c326d8e03e09
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

消耗錯誤並增加類別。

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 參數、回傳值及常規用法

kind: ErrorKind；消耗 self 並回傳新 DecimalError。保留原類別和訊息，忽略重複類別。原瀏覽器控制代碼失效。無效 JS 類別在移動前被拒絕。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
