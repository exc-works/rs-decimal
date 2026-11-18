---
locale: zh-Hans
source_revision: cee1a90bda7ee828acf5a855ac4b338f7c02ca9ef05b0b3742b98db12c7cf78a
api_id: "NullDecimal::marshal_xml_attr"
---
# NullDecimal::marshal_xml_attr

获取 xml_attr 语义内容。

```rust
fn marshal_xml_attr (& self) -> Option < String >
```

## 参数、返回值及常规用法

无参数，返回 `Option<String>`/string | null。Rust 需要 xml feature，浏览器已启用。返回已解码的属性内容；调用者提供属性名并进行 XML 转义。规范小数文本移除小数末尾零。无效 nullable 返回 None/null：应省略属性，而不是输出空属性。内部零未初始化的有效值返回 "0"。读取不改变初始化或有效标志；浏览器将 Rust None 映射为 null。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml-attr" locale="zh-Hans" />

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
