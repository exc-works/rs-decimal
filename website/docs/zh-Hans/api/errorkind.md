---
locale: zh-Hans
source_revision: e6b8dfbe3d3140a4697ec5a94f4d508f975bc9c0847e91c70ef29e95978ddcfb
api_id: "ErrorKind"
---
# ErrorKind

描述稳定的库错误类别。

```rust
pub enum ErrorKind
```

## 参数、返回值及常规用法

枚举包含十一个变体，在 Rust 中可复制、排序。浏览器 ErrorKind 是从 Rust 加载的冻结名称映射。处理特定错误时检查类别成员关系，不比较消息。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved JavaScript 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
