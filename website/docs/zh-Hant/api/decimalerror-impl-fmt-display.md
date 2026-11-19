---
locale: zh-Hant
source_revision: b479610db2fc4071c7650b3bc2c08969f87f046ec9147a92c1fba0a74d41a23f
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

使用 Rust fmt  實現。

```rust
impl fmt :: Display for DecimalError
```

## 參數、回傳值及常規用法

Display 只寫訊息。使用 format! 或 to_string()；機器處理應另行檢查 kinds。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
