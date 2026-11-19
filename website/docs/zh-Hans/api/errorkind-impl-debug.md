---
locale: zh-Hans
source_revision: 1b410a44c40ddf1ed7ab08ecafd94bf955a8c6e8ace55716beba7530f0db9122
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for ErrorKind
```

## 参数、返回值及常规用法

Debug 提供诊断文本，不是稳定的传输序列化。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
