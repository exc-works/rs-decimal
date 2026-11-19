---
locale: zh-Hant
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

使用 Rust Copy 實現。

```rust
impl Copy for ErrorKind
```

## 參數、回傳值及常規用法

Copy 複製此小型列舉，不深拷貝借用的純量資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
