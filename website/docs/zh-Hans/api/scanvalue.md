---
locale: zh-Hans
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
api_id: "ScanValue"
---
# ScanValue

选择与驱动无关的 SQL 输入类型。

```rust
pub enum ScanValue< 'a >
```

## 参数、返回值及常规用法

此枚举区分 NULL、f32、f64、i64、文本、文本字节和不支持的值。调用 Decimal.scan 或 NullDecimal.scan 前明确选择变体。Rust 借用的文本/字节在调用期间有效即可；浏览器 Bytes 保留调用者数组，并观察 scan 前的修改。不连接数据库服务。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
