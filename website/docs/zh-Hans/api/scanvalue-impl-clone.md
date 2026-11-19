---
locale: zh-Hans
source_revision: 11ee599a7357841e984cdaecadf4da635302996e14ae33191c323e57e59262ff
api_id: "ScanValue::impl Clone"
---
# ScanValue::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for ScanValue
```

## 参数、返回值及常规用法

Clone 保留完整值；ScanValue 的借用切片仍引用原数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-clone" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
