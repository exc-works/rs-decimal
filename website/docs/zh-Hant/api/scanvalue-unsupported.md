---
locale: zh-Hant
source_revision: 174ec0afec71ca2dec4f0c9c844589a1e01782200d059f787f800837923cca1b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

選擇 Unsupported SQL 輸入變體。

```rust
Unsupported
```

## 參數、回傳值及常規用法

不做強制轉換，明確報告不支援的驅動輸入。scan 回傳 Unmarshal 並保留原值。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
