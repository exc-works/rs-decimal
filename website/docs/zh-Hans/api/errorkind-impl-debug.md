---
locale: zh-Hans
source_revision: 8a9067139f7128cd65beb0f6ffcecf849b824634ed652fd6778b7004ed84c520
api_id: "ErrorKind::impl Debug"
---
# ErrorKind::impl Debug

使用 Rust Debug 实现。

```rust
impl Debug for ErrorKind
```

## 参数、返回值及常规用法

Debug 提供诊断文本，不是稳定的传输序列化。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-debug" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
