---
locale: zh-Hant
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
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

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
