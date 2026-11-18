---
locale: zh-Hans
source_revision: eede40b6a84e8fed75b7b014beb0ed7d44fa48b5c63abef14a875a6d37524b2d
api_id: "ErrorKind::impl PartialEq"
---
# ErrorKind::impl PartialEq

使用 Rust PartialEq 实现。

```rust
impl PartialEq for ErrorKind
```

## 参数、返回值及常规用法

相等比较完整错误消息和类别，或确切枚举变体。DecimalError 相等比匹配一个类别更严格。这是原生 Rust trait 语法，浏览器不将其暴露为方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-partialeq" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
