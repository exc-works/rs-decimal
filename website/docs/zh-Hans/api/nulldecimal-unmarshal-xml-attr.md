---
locale: zh-Hans
source_revision: f9048a3e7634cd109eea9581928de5e077f7c7324b6f3b621f732137009253fb
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

将 xml_attr 解码到现有值。

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

value: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 xml。传入已完成实体解码的属性值，而不是 XML 属性片段："1.23" 有效，但 `"&#49;.23"` 不会再次解码。去除首尾空白后精确解析小数文本。空内容清除 valid 并将内部 Decimal 重置为未初始化零。成功解码非空值会初始化小数，并保留非零值的末尾 scale。同时设置 valid=true。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
