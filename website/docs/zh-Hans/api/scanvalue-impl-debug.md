---
locale: zh-Hans
source_revision: ff9a08f2b3feca78c7b4250dd2c69ee67ba05009b21ca77af28e0824e74ae8d9
api_id: "ScanValue::impl Debug"
---
# ScanValue::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for ScanValue
```

## 参数、返回值及常规用法

Debug 提供诊断文本，不是稳定的传输序列化。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-debug" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
