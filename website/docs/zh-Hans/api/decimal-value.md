---
locale: zh-Hans
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
api_id: "Decimal::value"
---
# Decimal::value

获取规范的 SQL 值。

```rust
fn value (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。即使 Decimal 未初始化也返回非 NULL 的 "0"；需要 SQL NULL 时使用 NullDecimal。移除小数末尾零，但不改变 scale 或初始化状态。返回文本是标量值，不是带引号的 SQL 查询片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
