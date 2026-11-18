---
locale: zh-Hans
source_revision: 753b858d717076ef46e98ae0c0b43697fb31b0651286396a37e7378758f67207
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

## 浏览器边界

解码输入最多 8192 字节，参数字符串还在复制到 Wasm 前检查长度。Rust 对文本及 JSON 解码后的字符串再次检查 [十进制资源限制](../resource-policy)，因此转义字符不能绕过指数展开或精度限制。无效 UTF-8 和 JSON/小数语法仍由核心报告。输入错误和资源错误都不会部分更新接收者；已移动的对象不可再读写。

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
