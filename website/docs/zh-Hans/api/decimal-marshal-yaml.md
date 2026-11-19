---
locale: zh-Hans
source_revision: 25be20babe80f8d8b33e901842da879899c54ff8a78f97427ea80f9ca18a005d
api_id: "Decimal::marshal_yaml"
---
# Decimal::marshal_yaml

获取 yaml 语义内容。

```rust
fn marshal_yaml (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。Rust 需要 yaml feature，浏览器已启用。返回 YAML 语义字符串标量，不是序列化后的 YAML 文档。交给 YAML emitter 时按字符串处理，由 emitter 负责引号和标签。规范小数文本移除小数末尾零。默认未初始化零返回 "0"。读取不改变初始化或有效标志。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-decimal-marshal-yaml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
