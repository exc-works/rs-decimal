---
locale: zh-Hans
source_revision: b5003d3283929da01970193aca2fe128bcc80043185972b8b1e270211629771c
api_id: "Decimal::scan"
---
# Decimal::scan

读取与数据库驱动无关的 SQL 标量。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

value: ScanValue；修改接收者，返回 `Result<(), DecimalError>`/undefined。明确选择变体：Int64 使用精确 bigint，Float32 收窄为 binary32，Text/Bytes 保留原始小数数字。SQL 文本可带一对首尾双引号，只移除引号而不进行 JSON 反转义。NULL 将接收者重置为未初始化零。不支持的类型和非有限浮点数返回 Unmarshal，其他解码错误保留底层类别。所有错误均保留接收者状态。此适配器不需要数据库连接。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-scan" locale="zh-Hans" />

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
