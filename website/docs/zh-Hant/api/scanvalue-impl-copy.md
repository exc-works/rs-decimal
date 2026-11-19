---
locale: zh-Hant
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

使用 Rust Copy 實現。

```rust
impl Copy for ScanValue
```

## 參數、回傳值及常規用法

Copy 複製此小型列舉，不深拷貝借用的純量資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
