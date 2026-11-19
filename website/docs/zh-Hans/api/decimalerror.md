---
locale: zh-Hans
source_revision: 8805baa34ac15d1bc6208bedc88ae9d1d05c8235150f9f2331a9105561b652f3
api_id: "DecimalError"
---
# DecimalError

保留消息以及一个或多个库错误类别。

```rust
pub struct DecimalError
```

## 参数、返回值及常规用法

DecimalError 保存可读消息和有序去重的类别。with_kind 消耗值，matches 和 kinds 借用值。它实现 Rust Error 与 Display。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
