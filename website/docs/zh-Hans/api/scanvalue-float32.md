---
locale: zh-Hans
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
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

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
