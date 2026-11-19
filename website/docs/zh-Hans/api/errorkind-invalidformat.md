---
locale: zh-Hans
source_revision: 72523519c8ccf1aad4c8baebb6ac1ef42e8070b9cc7f7f72b600ff77781eaf4e
api_id: "ErrorKind::InvalidFormat"
---
# ErrorKind::InvalidFormat

识别 InvalidFormat 错误。

```rust
InvalidFormat
```

## 参数、返回值及常规用法

输入不是受支持的有限小数表示。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidformat" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
