---
locale: zh-Hans
source_revision: dbb045138dac94f7b2691e8576e6af500d5ead689869256e178b5069283ab80d
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

选择 Float32 SQL 输入变体。

```rust
Float32 (f32)
```

## 参数、返回值及常规用法

传递 binary32。浏览器接受 number，Rust 在 scan 前将其收窄为 f32。NaN、无穷和收窄溢出返回 Unmarshal。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
