---
locale: zh-Hans
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
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

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
