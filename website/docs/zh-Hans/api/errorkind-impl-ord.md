---
locale: zh-Hans
source_revision: 93c9a590df79625539c288c9f6a42d27aa1dba571b28363b98f9ec4642104e16
api_id: "ErrorKind::impl Ord"
---
# ErrorKind::impl Ord

使用 Rust Ord 实现。

```rust
impl Ord for ErrorKind
```

## 参数、返回值及常规用法

按 Rust 声明顺序排列 ErrorKind，不按严重程度或名称字母序。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-ord" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
