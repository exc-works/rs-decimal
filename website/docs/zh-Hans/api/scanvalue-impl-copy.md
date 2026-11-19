---
locale: zh-Hans
source_revision: 7eeabc8ee1ace4f862b185d7c83f7d0a53594b9ff67ac15d988dde4ffaa12e4b
api_id: "ScanValue::impl Copy"
---
# ScanValue::impl Copy

使用 Rust Copy 实现。

```rust
impl Copy for ScanValue
```

## 参数、返回值及常规用法

Copy 复制此小型枚举，不深拷贝借用的标量数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-impl-copy" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
