---
locale: zh-Hant
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

使用 Rust PartialOrd 實現。

```rust
impl PartialOrd for ErrorKind
```

## 參數、回傳值及常規用法

按 Rust 聲明順序排列 ErrorKind，不按嚴重程度或名稱字母序。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
