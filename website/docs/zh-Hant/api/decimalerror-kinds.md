---
locale: zh-Hant
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

讀取全部錯誤類別。

```rust
fn kinds (& self) -> & [ErrorKind]
```

## 參數、回傳值及常規用法

無參數，回傳 &[ErrorKind]/獨立的變體名陣列。類別去重後按 Rust 列舉順序排序，不是字母序。瀏覽器具名建構函數建立可釋放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。拋出的程式庫異常是原生 JS Error，帶 category="library" 和 kinds 陣列；此時使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 錯誤與程式庫錯誤分開，不帶程式庫類別。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
