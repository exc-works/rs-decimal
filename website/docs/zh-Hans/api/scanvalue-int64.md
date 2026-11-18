---
locale: zh-Hans
source_revision: 36f5ff0720c14718444a86498f3f5cd1d16ba0df1d4c60c2d59357d8e469675e
api_id: "ScanValue::Int64"
---
# ScanValue::Int64

选择 Int64 SQL 输入变体。

```rust
Int64 (i64)
```

## 参数、返回值及常规用法

精确传递 i64。浏览器要求有符号 64 位范围内的 bigint；拒绝 JS number。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-int64" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
