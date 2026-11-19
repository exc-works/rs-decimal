---
locale: zh-Hant
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

識別 Unmarshal 錯誤。

```rust
Unmarshal
```

## 參數、回傳值及常規用法

解碼失敗。InvalidFormat 等其他類別可能說明原因；應檢查類別成員關係，不能假設僅有一種類別。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
