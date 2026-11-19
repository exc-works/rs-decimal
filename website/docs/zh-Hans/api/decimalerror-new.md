---
locale: zh-Hans
source_revision: 52cfb6e6f9d640e445d0e6b774455d4a9cdc44bf1d1bc94d3efd717749548fc4
api_id: "DecimalError::new"
---
# DecimalError::new

创建包含一个类别的错误。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 参数、返回值及常规用法

kind: ErrorKind，message: impl `Into<String>`/string；返回新 DecimalError。消息供人阅读，不决定类别成员关系。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
