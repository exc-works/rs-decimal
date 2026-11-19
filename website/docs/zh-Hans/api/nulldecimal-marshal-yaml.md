---
locale: zh-Hans
source_revision: 634992be26686e49442a948ce62a476a4bcc34db498b408945e4ad08ce9f7530
api_id: "NullDecimal::marshal_yaml"
---
# NullDecimal::marshal_yaml

获取 yaml 语义内容。

```rust
fn marshal_yaml (& self) -> Option < String >
```

## 参数、返回值及常规用法

无参数，返回 `Option<String>`/string | null。Rust 需要 yaml feature，浏览器已启用。返回 YAML 语义字符串标量，不是序列化后的 YAML 文档。交给 YAML emitter 时按字符串处理，由 emitter 负责引号和标签。规范小数文本移除小数末尾零。无效 nullable 返回 None/null，表示 YAML null 标量；内部零未初始化的有效值返回 "0"。读取不改变初始化或有效标志；浏览器将 Rust None 映射为 null。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-marshal-yaml" locale="zh-Hans" />

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
