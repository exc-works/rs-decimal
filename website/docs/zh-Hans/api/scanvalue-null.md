---
locale: zh-Hans
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
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

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
