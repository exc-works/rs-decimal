---
locale: zh-Hans
source_revision: c50c700bf613d916c3c5e5c98c1711b754a7a2e86cbb6f64e10b64167cb79a2a
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

识别 NoConvergence 错误。

```rust
NoConvergence
```

## 参数、返回值及常规用法

有界算法无法确认舍入结果；不表示数学结果不存在。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
