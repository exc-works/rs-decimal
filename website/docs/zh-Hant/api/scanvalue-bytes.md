---
locale: zh-Hant
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

選擇 Bytes SQL 輸入變體。

```rust
Bytes (& 'a [u8])
```

## 參數、回傳值及常規用法

借用小數文字位元組，不是 binary/BSON 編碼。瀏覽器將 Uint8Array 保留至 scan，之前的修改會被觀察到。scan 不修改輸入。無效 UTF-8 導致小數解析失敗。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
