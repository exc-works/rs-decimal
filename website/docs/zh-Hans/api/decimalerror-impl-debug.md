---
locale: zh-Hans
source_revision: f91df84f154c9cc1b9dd7b8a5909be0f1702eb57d28ff7a024367e2c25d73638
api_id: "DecimalError::impl Debug"
---
# DecimalError::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for DecimalError
```

## 参数、返回值及常规用法

Debug 提供诊断文本，不是稳定的传输序列化。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-debug" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
