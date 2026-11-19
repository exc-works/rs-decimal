---
locale: zh-Hant
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for ScanValue
```

## 參數、回傳值及常規用法

Clone 保留完整值；ScanValue 的借用切片仍參照原資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
