---
locale: zh-Hans
source_revision: e14d22e2fbe70e02eda750145ed5b1a70252e171e63aeaafcfc01281be24e818
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

识别 InvalidArgument 错误。

```rust
InvalidArgument
```

## 参数、返回值及常规用法

参数、验证规则或消息模板无效。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
