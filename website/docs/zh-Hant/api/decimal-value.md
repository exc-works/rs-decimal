---
locale: zh-Hant
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::value"
---
# Decimal::value

取得規範的 SQL 值。

```rust
fn value (& self) -> String
```

## 參數、回傳值及常規用法

無參數，回傳 String/string。即使 Decimal 未初始化也回傳非 NULL 的 "0"；需要 SQL NULL 時使用 NullDecimal。移除小數末尾零，但不改變 scale 或初始化狀態。回傳文字是純量值，不是帶引號的 SQL 查詢片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
