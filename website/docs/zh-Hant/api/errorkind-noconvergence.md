---
locale: zh-Hant
source_revision: c50c700bf613d916c3c5e5c98c1711b754a7a2e86cbb6f64e10b64167cb79a2a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

識別 NoConvergence 錯誤。

```rust
NoConvergence
```

## 參數、回傳值及常規用法

有界演算法無法確認捨入結果；不表示數學結果不存在。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
