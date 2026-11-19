---
locale: zh-Hans
source_revision: e2bc3815435c7e993973dc27c3f32515fd48346d012f56633ed09feff4f0aa99
api_id: "ErrorKind::NegativeRoot"
---
# ErrorKind::NegativeRoot

识别 NegativeRoot 错误。

```rust
NegativeRoot
```

## 参数、返回值及常规用法

对负数请求偶次根。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-negativeroot" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
