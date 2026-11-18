---
locale: zh-Hans
source_revision: 8e304b6c42ea7cec512b7054ed8c7c7654d4540ebdd19dbc647e83dc885c5809
api_id: "ScanValue::Null"
---
# ScanValue::Null

选择 Null SQL 输入变体。

```rust
Null
```

## 参数、返回值及常规用法

表示 SQL NULL；scan 将 Decimal 重置为未初始化零，并清除 NullDecimal。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
