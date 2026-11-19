---
locale: zh-Hant
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

識別 InvalidLog 錯誤。

```rust
InvalidLog
```

## 參數、回傳值及常規用法

對數輸入不是正數。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
