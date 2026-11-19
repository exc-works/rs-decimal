---
locale: zh-Hans
source_revision: aeb152b6bffb5f406cc2fbe512dd2449186121fa1cd03efbf5d74f0d3891aed1
api_id: "ErrorKind::NoConvergence"
---
# ErrorKind::NoConvergence

识别 NoConvergence 错误。

```rust
NoConvergence
```

## 参数、返回值及常规用法

有界算法无法确认舍入结果；不表示数学结果不存在。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-noconvergence" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
