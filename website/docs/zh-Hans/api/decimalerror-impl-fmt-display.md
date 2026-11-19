---
locale: zh-Hans
source_revision: b479610db2fc4071c7650b3bc2c08969f87f046ec9147a92c1fba0a74d41a23f
api_id: "DecimalError::impl fmt :: Display"
---
# DecimalError::impl fmt :: Display

使用 Rust fmt  实现。

```rust
impl fmt :: Display for DecimalError 
```

## 参数、返回值及常规用法

Display 只写消息。使用 format! 或 to_string()；机器处理应另行检查 kinds。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
