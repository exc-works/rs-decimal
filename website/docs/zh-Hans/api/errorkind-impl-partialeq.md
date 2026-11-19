---
locale: zh-Hans
source_revision: 5ee0e7b51aa068361991bd8a8b6016ae9f25f271ec57ec1d0990b39445f68b64
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

使用 Rust PartialEq 实现。

```rust
impl PartialEq for ErrorKind
```

## 参数、返回值及常规用法

相等比较完整错误消息和类别，或确切枚举变体。DecimalError 相等比匹配一个类别更严格。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
