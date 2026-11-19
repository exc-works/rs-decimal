---
locale: zh-Hant
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
translation_provenance: zh-Hans-terminology-reviewed
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

使用 Rust Clone 實現。

```rust
impl Clone for DecimalError
```

## 參數、回傳值及常規用法

Clone 保留完整值；ScanValue 的借用切片仍參照原資料。這是原生 Rust trait 語法，瀏覽器不將其暴露為方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
