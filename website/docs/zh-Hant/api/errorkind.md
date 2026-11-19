---
locale: zh-Hant
source_revision: 3bfbb5926961d9d892e3da2738bc0fc6133b23f056d8610803f8a02d24a533d4
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind"
---
# ErrorKind

描述穩定的程式庫錯誤類別。

```rust
pub enum ErrorKind
```

## 參數、回傳值及常規用法

列舉包含十一個變體，在 Rust 中可複製、排序。瀏覽器 ErrorKind 是從 Rust 載入的凍結名稱映射。處理特定錯誤時檢查類別成員關係，不比較訊息。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
