---
locale: zh-Hans
source_revision: c136bea39fff64039ef208c269a0655f2c126075df944740bc4d24f9d953ef55
api_id: "ErrorKind"
---
# ErrorKind

描述稳定的库错误类别。

```rust
pub enum ErrorKind
```

## 参数、返回值及常规用法

枚举包含十一个变体，在 Rust 中可复制、排序。浏览器 ErrorKind 是从 Rust 加载的冻结名称映射。处理特定错误时检查类别成员关系，不比较消息。浏览器具名构造函数创建可释放的 Rust 值，提供 message、matches()、kinds() 和 with_kind()。抛出的库异常是原生 JS Error，带 category="library" 和 kinds 数组；此时使用 e.kinds.includes(ErrorKind.Unmarshal)。parameter/resource/moved bridge 错误与库错误分开，不带库类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
