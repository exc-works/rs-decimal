---
locale: zh-Hant
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

選擇 Float32 SQL 輸入變體。

```rust
Float32 (f32)
```

## 參數、回傳值及常規用法

傳遞 binary32。瀏覽器接受 number，Rust 在 scan 前將其收窄為 f32。NaN、無窮和收窄溢出回傳 Unmarshal。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="zh-Hant" />

## 瀏覽器邊界

SQL 文字及位元組輸入、錯誤訊息最多 8192 個 UTF-8 位元組。SQL 去除外圍引號後的文字仍經過 [十進位資源檢查](../resource-policy)，不能借引號繞過指數限制。借用操作不消耗輸入；消耗後或釋放後的控制代碼不能重複使用。原生 Rust trait 和匯入範例由原生測試執行，不計作瀏覽器方法。

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
