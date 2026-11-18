---
locale: zh-Hans
source_revision: c7a1930837be67e808eeab76b8c26f44a4383de987cd691171dbb7ac852d0e40
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

将 xml_attr 解码到现有值。

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

value: &str/string；修改接收者，返回 `Result<(), DecimalError>`/undefined。Rust 需要 xml。传入已完成实体解码的属性值，而不是 XML 属性片段："1.23" 有效，但 "&#49;.23" 不会再次解码。去除首尾空白后精确解析小数文本。空内容清除 valid 并将内部 Decimal 重置为未初始化零。成功解码非空值会初始化小数，并保留非零值的末尾 scale。同时设置 valid=true。错误保留接收者全部字段，并保留包含 Unmarshal 的库错误类别。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="zh-Hans" />

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
