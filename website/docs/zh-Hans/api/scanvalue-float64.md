---
locale: zh-Hans
source_revision: b276bcddff5985f20e7905654e4b3cef6f352b3cfa139550a92b9425bd777e62
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

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
