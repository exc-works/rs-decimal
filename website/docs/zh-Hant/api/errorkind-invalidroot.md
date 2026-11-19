---
locale: zh-Hant
source_revision: 5acf1a9800c0d02bd1cb1fa68c2c1f7068c1bb36cf20034e0a1e404758ce2bc4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::InvalidRoot"
---
# ErrorKind::InvalidRoot

識別 InvalidRoot 錯誤。

```rust
InvalidRoot
```

## 參數、回傳值及常規用法

根次數不是正數。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidroot" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
