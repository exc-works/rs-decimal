---
locale: zh-Hans
source_revision: 8ab6c44a0e4cf327b955869493409ea9bcd447a1bc1451590926853f00b0ca0b
api_id: "Decimal::unmarshal_json"
---
# Decimal::unmarshal_json

将 json 输入解码到现有 Decimal。

```rust
fn unmarshal_json (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

bytes: &[u8]/Uint8Array；修改接收者，返回 Result<(), DecimalError>/undefined。Rust 需要 json feature。传入原始 UTF-8 JSON 字节：数字不经过浮点数，字符串先反转义再精确解析小数。不要先把数字 token 用 JSON.parse 解析为 JS number。JSON 空白仅包括空格、tab、CR、LF。JSON null 保留现有 Decimal 的完整状态，包括初始化状态。成功解析非空值会初始化小数，保留非零值的末尾精度，并将解析得到的零规范化为 scale 零。错误保留接收者的完整状态。库解码错误保留 Unmarshal 及底层类别；资源错误同样发生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-json" locale="zh-Hans" />

## 浏览器边界

解码输入最多 8192 字节，参数字符串还在复制到 Wasm 前检查长度。Rust 对文本及 JSON 解码后的字符串再次检查 [十进制资源限制](../resource-policy)，因此转义字符不能绕过指数展开或精度限制。无效 UTF-8 和 JSON/小数语法仍由核心报告。输入错误和资源错误都不会部分更新接收者；已移动的对象不可再读写。

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
