---
locale: zh-Hant
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

使用 Rust PartialEq 實現。

```rust
impl PartialEq for ErrorKind
```

## 參數、回傳值及常規用法

相等比較完整錯誤訊息和類別，或確切列舉變體。DecimalError 相等比匹配一個類別更嚴格。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
