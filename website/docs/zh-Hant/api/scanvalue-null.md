---
locale: zh-Hant
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

選擇 Null SQL 輸入變體。

```rust
Null
```

## 參數、回傳值及常規用法

表示 SQL NULL；scan 將 Decimal 重置為未初始化零，並清除 NullDecimal。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
