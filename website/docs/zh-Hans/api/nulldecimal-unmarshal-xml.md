---
locale: zh-Hans
source_revision: e94c40441044730df649ba261de89daead4c5ce018d30a42e07e0ff558def35c
api_id: "NullDecimal::unmarshal_xml"
---
# NullDecimal::unmarshal_xml

将 xml 解码到现有值。

```rust
fn unmarshal_xml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

source: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 xml。解码首个 XML 元素，组合直接文本、CDATA 和解码后的实体引用。忽略嵌套子元素的文本，并连接子元素前后的直接文本。忽略后续元素。核心验证名称、属性、字符引用和匹配的结束标签。去除首尾空白后内容为空时清除 valid 并将内部 Decimal 重置为未初始化零。成功解码非空值会初始化小数，并保留非零值的末尾 scale。同时设置 valid=true。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
