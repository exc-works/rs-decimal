---
locale: zh-Hans
source_revision: e1a6c7014c208ee06566822a33e4d32bdbca21d8076f5cc955ee91bcd5ae501d
api_id: "NullDecimal::unmarshal_yaml"
---
# NullDecimal::unmarshal_yaml

将 yaml 解码到现有值。

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

source: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 yaml。传入原始 YAML 源文本，使 Rust 能处理引号、标签和精确数字。普通 012 按八进制解析为 10，而带引号的 "012" 或 !!str 012 解析为十进制 12。数字标签、进制和下划线遵循核心解析规则：普通 1_000 为 1000，但带引号的 "1_000" 无效。布尔值、集合和块标量被拒绝。仅首个文档提供值，但后续文档仍须语法有效。null 或空文档清除 valid 并将内部 Decimal 重置为未初始化零。成功解码非空值会初始化小数，并保留非零值的末尾 scale。同时设置 valid=true。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="zh-Hans" />

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
