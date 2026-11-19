---
locale: zh-Hans
source_revision: bd62f5fbc640a37b2adf289a5796e4f02bc2e94ecde07c9c595d96aa3787e2b3
api_id: "ErrorKind::Overflow"
---
# ErrorKind::Overflow

识别 Overflow 错误。

```rust
Overflow
```

## 参数、返回值及常规用法

系数、scale 或运算超过核心支持范围。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-overflow" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
