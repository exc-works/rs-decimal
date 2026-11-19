---
locale: zh-Hans
source_revision: 001b8b003c099ab66879c97c5d63d2105fa698bcbc3c15c760670300d17a20a3
api_id: "ErrorKind::impl Eq"
---
# ErrorKind::impl Eq

使用 Rust Eq 实现。

```rust
impl Eq for ErrorKind
```

## 参数、返回值及常规用法

相等比较完整错误消息和类别，或确切枚举变体。DecimalError 相等比匹配一个类别更严格。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-eq" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
