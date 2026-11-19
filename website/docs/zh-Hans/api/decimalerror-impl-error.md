---
locale: zh-Hans
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

使用 Rust Error 实现。

```rust
impl Error for DecimalError 
```

## 参数、返回值及常规用法

作为标准 Rust Error 使用，包括装箱传播。附加类别不是链式 source，source() 为 None。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
