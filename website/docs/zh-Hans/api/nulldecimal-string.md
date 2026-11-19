---
locale: zh-Hans
source_revision: 5f7a180e14ebabfd6785a42604765a6ece20be12513e46794eb1f0299f911006
api_id: "NullDecimal::string"
---
# NullDecimal::string

读取 nullable 的规范文本。

```rust
fn string (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string。有效值使用内部 Decimal 的规范文本，移除小数末尾零。无效值始终返回字面 null，与隐藏的内部数据无关。这与 marshal_text 不同：后者对无效值返回空字节。读取不改变有效性或初始化状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-string" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
