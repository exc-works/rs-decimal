---
locale: zh-Hans
source_revision: 189b6d1ee3eb48467bf982330ec482f99ea7cd30e80f4f3c225f2808c0e03cc3
api_id: "ErrorKind::RoundUnnecessary"
---
# ErrorKind::RoundUnnecessary

识别 RoundUnnecessary 错误。

```rust
RoundUnnecessary
```

## 参数、返回值及常规用法

要求精确舍入，但会丢弃非零数字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-roundunnecessary" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
