---
locale: zh-Hans
source_revision: bceeb7ede6fc6a0450eec9bb9848cbe43ca4c0d990bf02edf9131f531001c34f
api_id: "DecimalError::impl PartialEq"
---
# DecimalError::impl PartialEq

使用 Rust PartialEq 实现。

```rust
impl PartialEq for DecimalError
```

## 参数、返回值及常规用法

相等比较完整错误消息和类别，或确切枚举变体。DecimalError 相等比匹配一个类别更严格。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-partialeq" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
