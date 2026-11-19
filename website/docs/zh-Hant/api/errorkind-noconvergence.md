---
locale: zh-Hant
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

識別 NoConvergence 錯誤。

```rust
NoConvergence
```

## 參數、回傳值及常規用法

有界演算法無法確認捨入結果；不表示數學結果不存在。ErrorKind 是穩定類別，不是錯誤訊息文字。瀏覽器列舉值為 Rust 變體名；建構的錯誤使用 matches()，捕獲的 bridge 異常提供 kinds 陣列。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
