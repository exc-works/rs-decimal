---
locale: zh-Hans
source_revision: 634fa480d38c38e977bcb656a0df8ea6f905528b83e0983585b20f7e954e4020
api_id: "DecimalError::new"
---
# DecimalError::new

创建包含一个类别的错误。

```rust
fn new (kind : ErrorKind , message : impl Into < String >) -> Self
```

## 参数、返回值及常规用法

kind: ErrorKind，message: impl `Into<String>`/string；返回新 DecimalError。消息供人阅读，不决定类别成员关系。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-new" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
