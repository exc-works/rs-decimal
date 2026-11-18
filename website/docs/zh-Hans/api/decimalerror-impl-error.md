---
locale: zh-Hans
source_revision: d94c5a1a48ed0fbe522456d88bc4774ce805cace3b32e2a5428e2527dc3be2ce
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

使用 Rust Error 实现。

```rust
impl Error for DecimalError 
```

## 参数、返回值及常规用法

作为标准 Rust Error 使用，包括装箱传播。附加类别不是链式 source，source() 为 None。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
