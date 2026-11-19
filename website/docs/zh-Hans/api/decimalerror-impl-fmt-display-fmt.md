---
locale: zh-Hans
source_revision: 725ef7f08917b0c306a1f269aaa8e188bc4212046b8491281df04a8f33b0e006
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

使用 Rust fmt  实现。

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## 参数、返回值及常规用法

Display 只写消息。使用 format! 或 to_string()；机器处理应另行检查 kinds。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
