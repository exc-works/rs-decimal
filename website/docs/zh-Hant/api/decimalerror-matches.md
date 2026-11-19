---
locale: zh-Hant
source_revision: 1c3bd02bc78216d73cebed78381413908e61e5b9b730c528a5271415d50a2935
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::matches"
---
# DecimalError::matches

檢查類別成員關係。

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## 參數、回傳值及常規用法

kind: ErrorKind；回傳 bool/boolean。借用不改變錯誤。同一個錯誤可以匹配多個類別。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
