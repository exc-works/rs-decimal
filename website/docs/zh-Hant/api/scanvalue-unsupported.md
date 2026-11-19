---
locale: zh-Hant
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
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

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
