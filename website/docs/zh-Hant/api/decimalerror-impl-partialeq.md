---
locale: zh-Hant
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

使用 Rust PartialEq 實現。

```rust
impl PartialEq for DecimalError
```

## 參數、回傳值及常規用法

相等比較完整錯誤訊息和類別，或確切列舉變體。DecimalError 相等比匹配一個類別更嚴格。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
