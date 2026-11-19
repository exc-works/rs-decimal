---
locale: zh-Hant
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

取得規範的 SQL 值。

```rust
fn value (& self) -> Option < String >
```

## 參數、回傳值及常規用法

無參數，回傳 `Option<String>`/string | null。無效 nullable 回傳 None/null，與隱藏內部值無關。有效值回傳規範小數文字，內部零未初始化時也回傳 "0"。移除小數末尾零，但不改變 scale 或初始化狀態。回傳文字是純量值，不是帶引號的 SQL 查詢片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
