---
locale: zh-Hant
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

使用 Rust Debug 實現。

```rust
impl Debug for ErrorKind
```

## 參數、回傳值及常規用法

Debug 提供診斷文字，不是穩定的傳輸序列化。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
