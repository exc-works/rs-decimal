---
locale: zh-Hans
source_revision: b6a1d6732343e8fec8b5e4584b880016ecb9ffdc8cf16c927c04637de618fc19
api_id: "ErrorKind::impl PartialOrd"
---
# ErrorKind::impl PartialOrd

使用 Rust PartialOrd 实现。

```rust
impl PartialOrd for ErrorKind
```

## 参数、返回值及常规用法

按 Rust 声明顺序排列 ErrorKind，不按严重程度或名称字母序。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialord" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
