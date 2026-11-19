---
locale: zh-Hant
source_revision: 6eb93b97fb5de84f7620ea6f0aa7d8d8ce7bbe38e3bab65b24ca028dc1ed97cd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

識別 Overflow 錯誤。

```rust
Overflow
```

## 參數、回傳值及常規用法

係數、scale 或運算超過核心支援範圍。瀏覽器資源錯誤與此類別分開。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
