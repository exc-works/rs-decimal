---
locale: zh-Hans
source_revision: 9dbbcca20e9ec70948f275a66820b5aebd2bfa7f33ae4f382b6d22c025c682c3
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for DecimalError
```

## 参数、返回值及常规用法

Clone 保留完整值；ScanValue 的借用切片仍引用原数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
