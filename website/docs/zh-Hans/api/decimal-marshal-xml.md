---
locale: zh-Hans
source_revision: 5f41d4af42d404ec01165b3190db1cd2f96b9a84fa765bafbce34ae6b76aa2ef
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

## 浏览器边界

源输入最多 8192 个 UTF-8 字节。Rust 在核心数值解析之前检查解码后的首个 YAML 标量或 XML 元素文本，包含转义、CDATA、下划线归一化及 YAML 浮点补零。YAML 标量最多 4096 个十六进制数字字符，以限制进制整数分配；保守检查可能拒绝原生库可接受的输入。scale、系数和调用次数遵循 [资源限制](../resource-policy)。已移动的句柄不能再读写。

## 相关方法

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
