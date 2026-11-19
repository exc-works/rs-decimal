---
locale: zh-Hant
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

選擇 Int64 SQL 輸入變體。

```rust
Int64 (i64)
```

## 參數、回傳值及常規用法

精確傳遞 i64。瀏覽器要求有符號 64 位範圍內的 bigint；拒絕 JS number。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
