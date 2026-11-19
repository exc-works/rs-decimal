---
locale: zh-Hant
source_revision: ff9a08f2b3feca78c7b4250dd2c69ee67ba05009b21ca77af28e0824e74ae8d9
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

使用 Rust Debug 實現。

```rust
impl Debug for ScanValue
```

## 參數、回傳值及常規用法

Debug 提供診斷文字，不是穩定的傳輸序列化。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
