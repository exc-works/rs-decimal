---
locale: zh-Hant
source_revision: 725ef7f08917b0c306a1f269aaa8e188bc4212046b8491281df04a8f33b0e006
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

使用 Rust fmt  實現。

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## 參數、回傳值及常規用法

Display 只寫訊息。使用 format! 或 to_string()；機器處理應另行檢查 kinds。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
