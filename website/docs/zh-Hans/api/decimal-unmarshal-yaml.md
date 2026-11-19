---
locale: zh-Hans
source_revision: c4ab958e126151b59020a16b1317b5deb3282029f32339008d09669426a6a46f
api_id: "Decimal::unmarshal_yaml"
---
# Decimal::unmarshal_yaml

将 yaml 解码到现有值。

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

source: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 yaml。传入原始 YAML 源文本，使 Rust 能处理引号、标签和精确数字。普通 012 按八进制解析为 10，而带引号的 "012" 或 !!str 012 解析为十进制 12。数字标签、进制和下划线遵循核心解析规则：普通 1_000 为 1000，但带引号的 "1_000" 无效。布尔值、集合和块标量被拒绝。仅首个文档提供值，但后续文档仍须语法有效。null 或空文档保留现有接收者，包括初始化和 scale。成功解码非空值会初始化小数，并保留非零值的末尾 scale。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-yaml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
