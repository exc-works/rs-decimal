---
locale: zh-Hans
source_revision: 1c3bd02bc78216d73cebed78381413908e61e5b9b730c528a5271415d50a2935
api_id: "DecimalError::matches"
---
# DecimalError::matches

检查类别成员关系。

```rust
fn matches (& self , kind : ErrorKind) -> bool
```

## 参数、返回值及常规用法

kind: ErrorKind；返回 bool/boolean。借用不改变错误。同一个错误可以匹配多个类别。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-matches" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
