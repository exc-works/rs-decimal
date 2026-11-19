---
locale: zh-Hans
source_revision: 8ca020b2d56652a32165b4756446630d2a88be60a068a89e4f65ce218930abe5
api_id: "Decimal::marshal_xml_attr"
---
# Decimal::marshal_xml_attr

获取 xml_attr 语义内容。

```rust
fn marshal_xml_attr (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。Rust 需要 xml feature，浏览器已启用。返回已解码的属性内容；调用者提供属性名并进行 XML 转义。规范小数文本移除小数末尾零。未初始化零返回空内容；已初始化零返回 "0"。读取不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml-attr" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
