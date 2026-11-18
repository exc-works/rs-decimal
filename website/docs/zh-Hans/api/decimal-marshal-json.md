---
locale: zh-Hans
source_revision: ce7a1e6d51ea25aa6e22728a52c66d3bfd202ca324cd22c0d1d0545a351bced2
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

将 Decimal 编码为 json 字节。

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 参数、返回值及常规用法

无参数，返回 `Vec<u8>`/Uint8Array，内容为JSON 字节。Rust 需要 json feature，浏览器构建已启用。已初始化的 Decimal 编码为带引号的规范字符串；默认未初始化零编码为 JSON null。返回字节是独立副本，读取不改变状态。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="zh-Hans" />

## 浏览器边界

解码输入最多 8192 字节，参数字符串还在复制到 Wasm 前检查长度。Rust 对文本及 JSON 解码后的字符串再次检查 [十进制资源限制](../resource-policy)，因此转义字符不能绕过指数展开或精度限制。无效 UTF-8 和 JSON/小数语法仍由核心报告。输入错误和资源错误都不会部分更新接收者；已移动的对象不可再读写。

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
