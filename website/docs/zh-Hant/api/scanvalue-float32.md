---
locale: zh-Hant
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

選擇 Float32 SQL 輸入變體。

```rust
Float32 (f32)
```

## 參數、回傳值及常規用法

傳遞 binary32。瀏覽器接受 number，Rust 在 scan 前將其收窄為 f32。NaN、無窮和收窄溢出回傳 Unmarshal。scan 借用變體，同一次瀏覽器執行中可重複使用；拒絕偽造或其他執行實例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
