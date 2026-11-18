---
locale: zh-Hant
source_revision: c870cbfccba48189ea2c1f73c60ffc0835f379cdb13fc7e1adf9905fa90662b8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

識別 Unmarshal 錯誤。

```rust
Unmarshal
```

## 參數、回傳值及常規用法

解碼失敗。InvalidFormat 等其他類別可能說明原因；應檢查類別成員關係，不能假設僅有一種類別。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
