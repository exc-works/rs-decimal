---
locale: zh-Hans
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
api_id: "NullDecimal::value"
---
# NullDecimal::value

获取规范的 SQL 值。

```rust
fn value (& self) -> Option < String >
```

## 参数、返回值及常规用法

无参数，返回 `Option<String>`/string | null。无效 nullable 返回 None/null，与隐藏内部值无关。有效值返回规范小数文本，内部零未初始化时也返回 "0"。移除小数末尾零，但不改变 scale 或初始化状态。返回文本是标量值，不是带引号的 SQL 查询片段。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
