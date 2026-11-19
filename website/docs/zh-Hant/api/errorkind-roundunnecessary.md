---
locale: zh-Hant
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

識別 RoundUnnecessary 錯誤。

```rust
RoundUnnecessary
```

## 參數、回傳值及常規用法

要求精確捨入，但會丟棄非零數字。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
