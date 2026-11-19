---
locale: zh-Hans
source_revision: 473b8cc7f11457cef7cc926925c4f230754afa71c4dea38c85c3b954c8ca89fd
api_id: "ErrorKind::impl Copy"
---
# ErrorKind::impl Copy

使用 Rust Copy 实现。

```rust
impl Copy for ErrorKind
```

## 参数、返回值及常规用法

Copy 复制此小型枚举，不深拷贝借用的标量数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-copy" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
