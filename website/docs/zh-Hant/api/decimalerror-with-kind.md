---
locale: zh-Hant
source_revision: ecc694dc9095e4bf91fee90f15b14f69b82899c7811b1a6abfdda76dd5e06c56
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::with_kind"
---
# DecimalError::with_kind

消耗錯誤並增加類別。

```rust
fn with_kind (mut self , kind : ErrorKind) -> Self
```

## 參數、回傳值及常規用法

kind: ErrorKind；消耗 self 並回傳新 DecimalError。保留原類別和訊息，忽略重複類別。原瀏覽器控制代碼失效。無效 JS 類別在移動前被拒絕。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-with-kind" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
