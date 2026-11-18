---
locale: zh-Hant
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

選擇 Bytes SQL 輸入變體。

```rust
Bytes (& 'a [u8])
```

## 參數、回傳值及常規用法

借用小數文字位元組，不是 binary/BSON 編碼。瀏覽器將 Uint8Array 保留至 scan，之前的修改會被觀察到。scan 不修改輸入。無效 UTF-8 導致小數解析失敗。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
