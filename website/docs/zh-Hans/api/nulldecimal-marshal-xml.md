---
locale: zh-Hans
source_revision: 7b63356b05d421377bb8ec0a29f08c338eaab7596c00a0c507a722a2de09d089
api_id: "NullDecimal::marshal_xml"
---
# NullDecimal::marshal_xml

获取 xml 语义内容。

```rust
fn marshal_xml (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。Rust 需要 xml feature，浏览器已启用。返回元素字符内容；调用者提供元素名、命名空间并进行 XML 转义。规范小数文本移除小数末尾零。无效 nullable 返回空内容；内部零未初始化的有效值返回 "0"。读取不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-xml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
