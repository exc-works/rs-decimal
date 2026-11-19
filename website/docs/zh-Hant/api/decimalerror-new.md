---
locale: zh-Hant
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::new"
---
# DecimalError::new

建立包含一個類別的錯誤。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 參數、回傳值及常規用法

kind: ErrorKind，message: impl `Into<String>`/string；回傳新 DecimalError。訊息供人閱讀，不決定類別成員關係。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
