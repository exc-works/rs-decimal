---
locale: zh-Hant
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
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

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
