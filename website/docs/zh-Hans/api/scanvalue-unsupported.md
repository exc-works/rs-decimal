---
locale: zh-Hans
source_revision: 78277d1082dac8a98f79b7018413113c09afb0c224a6c82ae6c179432b0517f1
api_id: "ScanValue::Unsupported"
---
# ScanValue::Unsupported

选择 Unsupported SQL 输入变体。

```rust
Unsupported
```

## 参数、返回值及常规用法

不做强制转换，明确报告不支持的驱动输入。scan 返回 Unmarshal 并保留原值。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-unsupported" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
