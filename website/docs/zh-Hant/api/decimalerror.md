---
locale: zh-Hant
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError"
---
# DecimalError

保留訊息以及一個或多個程式庫錯誤類別。

```rust
pub struct DecimalError
```

## 參數、回傳值及常規用法

DecimalError 儲存可讀訊息和有序去重的類別。with_kind 消耗值，matches 和 kinds 借用值。它實現 Rust Error 與 Display。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
