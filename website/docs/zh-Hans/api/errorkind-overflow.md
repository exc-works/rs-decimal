---
locale: zh-Hans
source_revision: 6eb93b97fb5de84f7620ea6f0aa7d8d8ce7bbe38e3bab65b24ca028dc1ed97cd
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

识别 Overflow 错误。

```rust
Overflow
```

## 参数、返回值及常规用法

系数、scale 或运算超过核心支持范围。浏览器资源错误与此类别分开。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
