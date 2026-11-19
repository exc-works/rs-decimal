---
locale: zh-Hans
source_revision: 274359ec57012f0ef20672a2e83c98afd2d82df0c75391b519022c4925e09ca8
api_id: "ErrorKind::Unmarshal"
---
# ErrorKind::Unmarshal

识别 Unmarshal 错误。

```rust
Unmarshal
```

## 参数、返回值及常规用法

解码失败。InvalidFormat 等其他类别可能说明原因；应检查类别成员关系，不能假设仅有一种类别。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-unmarshal" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
