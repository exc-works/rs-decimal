---
locale: zh-Hans
source_revision: 1534eba12b3e65df06a4bea76e8dd9c9cd12eeb6bcfbce92f69f83b923eef4c3
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

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
