---
locale: zh-Hans
source_revision: bb5fda951c514c30196177535ed36df93d91d15de5889b17222eea8aae392724
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

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
