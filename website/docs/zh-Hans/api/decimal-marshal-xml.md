---
locale: zh-Hans
source_revision: 90631f865fb2217f0d8137591d3ab6e82b220d3c98120fa1ec15f91a894f2664
api_id: "Decimal::marshal_xml"
---
# Decimal::marshal_xml

获取 xml 语义内容。

```rust
fn marshal_xml (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。Rust 需要 xml feature，浏览器已启用。返回元素字符内容；调用者提供元素名、命名空间并进行 XML 转义。规范小数文本移除小数末尾零。未初始化零返回空内容；已初始化零返回 "0"。读取不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-xml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
