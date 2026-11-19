---
locale: zh-Hans
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

选择 Bytes SQL 输入变体。

```rust
Bytes (& 'a [u8])
```

## 参数、返回值及常规用法

借用小数文本字节，不是 binary/BSON 编码。浏览器将 Uint8Array 保留至 scan，之前的修改会被观察到。scan 不修改输入。无效 UTF-8 导致小数解析失败。scan 借用变体，同一次浏览器运行中可复用；拒绝伪造或其他运行实例的值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
