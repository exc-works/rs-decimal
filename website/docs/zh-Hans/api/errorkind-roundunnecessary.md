---
locale: zh-Hans
source_revision: dc7af33554badb1c95cd11064df3b51faeaca3112afc1b82cf2efcaf4ff1844d
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

识别 RoundUnnecessary 错误。

```rust
RoundUnnecessary
```

## 参数、返回值及常规用法

要求精确舍入，但会丢弃非零数字。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
