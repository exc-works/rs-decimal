---
locale: zh-Hant
source_revision: c5bc8b97bab18c557c0918fb7ad15cabc2fd824c618c469b8f007d1f6299272b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::DivideByZero"
---
# ErrorKind::DivideByZero

識別 DivideByZero 錯誤。

```rust
DivideByZero
```

## 參數、回傳值及常規用法

除數為零。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-dividebyzero" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
