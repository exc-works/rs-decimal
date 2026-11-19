---
locale: zh-Hant
source_revision: cbf84996c156999f0cfb9cac49a1135ecf0d19ca8aba3abbedbed0acff443c0a
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl Eq"
---
# DecimalError::impl Eq

使用 Rust Eq 實現。

```rust
impl Eq for DecimalError
```

## 參數、回傳值及常規用法

相等比較完整錯誤訊息和類別，或確切列舉變體。DecimalError 相等比匹配一個類別更嚴格。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-eq" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
