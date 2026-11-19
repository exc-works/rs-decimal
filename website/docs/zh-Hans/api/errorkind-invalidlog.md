---
locale: zh-Hans
source_revision: c456812ef50493713a2bbd81d6c57db5a0f730099b5878e83b3d4f358245d822
api_id: "ErrorKind::InvalidLog"
---
# ErrorKind::InvalidLog

识别 InvalidLog 错误。

```rust
InvalidLog
```

## 参数、返回值及常规用法

对数输入不是正数。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidlog" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
