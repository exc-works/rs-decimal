---
locale: zh-Hant
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: zh-Hans-terminology-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for ErrorKind
```

## 參數、回傳值及常規用法

Clone 保留完整值；ScanValue 的借用切片仍參照原資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
