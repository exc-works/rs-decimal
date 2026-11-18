---
locale: zh-Hant
source_revision: 65e698cc918f9afeb389a84627e95883beff1af27e6fb5183194df8bba07cbc9
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

識別 DivideByZero 錯誤。

```rust
DivideByZero
```

## 參數、回傳值及常規用法

除數為零。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
