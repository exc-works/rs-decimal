---
locale: zh-Hans
source_revision: b8d3cac3d5f9251c9158f0efb63e99a5a338182fad355d14b4c984c245247f04
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

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
