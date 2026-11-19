---
locale: zh-Hant
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

使用 Rust Ord 實現。

```rust
impl Ord for ErrorKind
```

## 參數、回傳值及常規用法

按 Rust 聲明順序排列 ErrorKind，不按嚴重程度或名稱字母序。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
