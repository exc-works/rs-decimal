---
locale: zh-Hans
source_revision: 2596ef8441aa7ec726abb87c7af62732ec4f2729da79754eba71b3cb01735596
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

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
