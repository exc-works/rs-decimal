---
locale: zh-Hant
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

識別 RoundUnnecessary 錯誤。

```rust
RoundUnnecessary
```

## 參數、回傳值及常規用法

要求精確捨入，但會丟棄非零數字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
