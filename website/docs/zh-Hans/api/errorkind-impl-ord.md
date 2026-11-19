---
locale: zh-Hans
source_revision: 77c1d26d73dd77cab1d0c87534f9e5ec0b5d57b39f76087cd3ef2b288f8c6fae
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

使用 Rust Ord 实现。

```rust
impl Ord for ErrorKind
```

## 参数、返回值及常规用法

按 Rust 声明顺序排列 ErrorKind，不按严重程度或名称字母序。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
