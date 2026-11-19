---
locale: zh-Hans
source_revision: a985525078f122e92ef2d70bf9a962a63b82b868ba67925f976407356bf4ff6b
api_id: "DecimalError::kinds"
---
# DecimalError::kinds

读取全部错误类别。

```rust
fn kinds (& self) -> & [ErrorKind]
```

## 参数、返回值及常规用法

无参数，返回 &[ErrorKind]/独立的变体名数组。类别去重后按 Rust 枚举顺序排序，不是字母序。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-kinds" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
