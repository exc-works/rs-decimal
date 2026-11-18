---
locale: zh-Hans
source_revision: 1f1d7bd844459ed8638e77e456d476b7cad16313d7e3492984a6370cba407445
api_id: "ErrorKind::InvalidArgument"
---
# ErrorKind::InvalidArgument

识别 InvalidArgument 错误。

```rust
InvalidArgument
```

## 参数、返回值及常规用法

参数、验证规则或消息模板无效。ErrorKind 是稳定类别，不是错误消息文本。浏览器枚举值为 Rust 变体名；构造的错误使用 matches()，捕获的 bridge 异常提供 kinds 数组。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-invalidargument" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
