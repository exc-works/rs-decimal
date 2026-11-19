---
locale: zh-Hant
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

使用 Rust Error 實現。

```rust
impl Error for DecimalError
```

## 參數、回傳值及常規用法

作為標準 Rust Error 使用，包括裝箱傳播。附加類別不是鏈式 source，source() 為 None。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
