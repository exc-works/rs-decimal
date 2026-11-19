---
locale: zh-Hans
source_revision: 66f48b07b8a5c080eb29b01047825ba3448ab4e979a526ae96e347e482fef177
api_id: "ScanValue::Text"
---
# ScanValue::Text

选择 Text SQL 输入变体。

```rust
Text (& 'a str)
```

## 参数、返回值及常规用法

借用精确小数文本。移除一对匹配的首尾双引号，但不按 JSON 解码转义。解析后保留非零值的末尾 scale。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-text" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
