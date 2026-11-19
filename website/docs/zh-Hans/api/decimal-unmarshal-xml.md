---
locale: zh-Hans
source_revision: 87ff1c64a9a30b8c362e1bf8438b7f751aeb4f561a2bd311095a9f2b86ed45db
api_id: "Decimal::unmarshal_xml"
---
# Decimal::unmarshal_xml

将 xml 解码到现有值。

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

source: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 xml。解码首个 XML 元素，组合直接文本、CDATA 和解码后的实体引用。忽略嵌套子元素的文本，并连接子元素前后的直接文本。忽略后续元素。核心验证名称、属性、字符引用和匹配的结束标签。去除首尾空白后内容为空时将接收者重置为未初始化零；这与保留原值的 YAML null 不同。成功解码非空值会初始化小数，并保留非零值的末尾 scale。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-unmarshal-xml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
