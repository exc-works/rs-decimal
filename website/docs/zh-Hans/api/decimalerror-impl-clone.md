---
locale: zh-Hans
source_revision: 401f5b53ed7c2747c248cad36ae59554a16f09a1f304c6c8d0b65cf689946b2f
api_id: "DecimalError::impl Clone"
---
# DecimalError::impl Clone

使用 Rust Clone 实现。

```rust
impl Clone for DecimalError
```

## 参数、返回值及常规用法

Clone 保留完整值；ScanValue 的借用切片仍引用原数据。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-clone" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
