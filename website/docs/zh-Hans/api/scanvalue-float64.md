---
locale: zh-Hans
source_revision: e5103a64d713be93c5c51f691b143dc290327bd825e0acd2029d936b2165437e
api_id: "ScanValue::Float64"
---
# ScanValue::Float64

选择 Float64 SQL 输入变体。

```rust
Float64 (f64)
```

## 参数、返回值及常规用法

传递 binary64。有限值使用核心最短十进制转换；NaN 和无穷返回 Unmarshal。已舍入的 JS number 无法恢复原始数字。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float64" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
