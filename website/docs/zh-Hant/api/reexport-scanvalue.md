---
locale: zh-Hant
source_revision: 8106ef71942ee657518a42cba33a247697b1ac2ebfcf011160e32c4adeb6e0b5
translation_provenance: zh-Hans-terminology-reviewed
api_id: "reexport::ScanValue"
---
# reexport::ScanValue

從 crate 根路徑匯入 ScanValue。

```rust
pub use sql::ScanValue as ScanValue
```

## 參數、回傳值及常規用法

透過 rs_decimal::ScanValue 存取同一公開條目。這是匯入路徑，不是額外實現或瀏覽器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-reexport-scanvalue" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
