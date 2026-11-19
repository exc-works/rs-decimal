---
locale: zh-Hant
source_revision: 001b8b003c099ab66879c97c5d63d2105fa698bcbc3c15c760670300d17a20a3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

使用 Rust Eq 實現。

```rust
impl Eq for ErrorKind
```

## 參數、回傳值及常規用法

相等比較完整錯誤訊息和類別，或確切列舉變體。DecimalError 相等比匹配一個類別更嚴格。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
