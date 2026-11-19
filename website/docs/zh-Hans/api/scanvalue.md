---
locale: zh-Hans
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
api_id: "ScanValue"
---
# ScanValue

选择与驱动无关的 SQL 输入类型。

```rust
pub enum ScanValue< 'a >
```

## 参数、返回值及常规用法

此枚举区分 NULL、f32、f64、i64、文本、文本字节和不支持的值。调用 Decimal.scan 或 NullDecimal.scan 前明确选择变体。Rust 借用的文本/字节在调用期间有效即可；浏览器 Bytes 保留调用者数组，并观察 scan 前的修改。不连接数据库服务。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
