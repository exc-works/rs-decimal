---
locale: zh-Hant
source_revision: 68b84662c55e7239d15b1fe54a5a2076eb45cea41916719d90f7dbe8ab9e7150
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::InvalidPrecision"
---
# ErrorKind::InvalidPrecision

識別 InvalidPrecision 錯誤。

```rust
InvalidPrecision
```

## 參數、回傳值及常規用法

請求的精度無效。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidprecision" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
