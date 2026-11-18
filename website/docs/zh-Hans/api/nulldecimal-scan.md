---
locale: zh-Hans
source_revision: 559007df6751f5417671f4f4b9a33c4ae6759ecf82dbd4fbeb7d9f0c40061e1a
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

读取与数据库驱动无关的 SQL 标量。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

value: ScanValue；修改接收者，返回 `Result<(), DecimalError>`/undefined。明确选择变体：Int64 使用精确 bigint，Float32 收窄为 binary32，Text/Bytes 保留原始小数数字。SQL 文本可带一对首尾双引号，只移除引号而不进行 JSON 反转义。NULL 清除有效标记并将内部 Decimal 重置为未初始化零；非空输入成功后设置 valid=true。不支持的类型和非有限浮点数返回 Unmarshal，其他解码错误保留底层类别。所有错误均保留接收者状态。此适配器不需要数据库连接。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="zh-Hans" />

## 浏览器边界

SQL 文本及字节输入、错误消息最多 8192 个 UTF-8 字节。SQL 去除外围引号后的文本仍经过 [十进制资源检查](../resource-policy)，不能借引号绕过指数限制。借用操作不消耗输入；消耗后或释放后的句柄不能复用。原生 Rust trait 和导入示例由原生测试执行，不计作浏览器方法。

## 相关方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
