---
locale: zh-Hans
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for ErrorKind
```

## 参数、返回值及常规用法

Clone 保留完整值；ScanValue 的借用切片仍引用原数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
