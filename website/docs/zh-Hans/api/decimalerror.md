---
locale: zh-Hans
source_revision: 51df12e136a0b4ed98ea9f557c4e61c7872827fb92cbe6a97475d66177b43284
api_id: "DecimalError"
---
# DecimalError

保留消息以及一个或多个库错误类别。

```rust
pub struct DecimalError
```

## 参数、返回值及常规用法

DecimalError 保存可读消息和有序去重的类别。with_kind 消耗值，matches 和 kinds 借用值。它实现 Rust Error 与 Display。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
