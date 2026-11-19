---
locale: zh-Hant
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

識別 InvalidFormat 錯誤。

```rust
InvalidFormat
```

## 參數、回傳值及常規用法

輸入不是受支援的有限小數表示。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
