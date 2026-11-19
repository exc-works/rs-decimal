---
locale: zh-Hans
source_revision: b2646b52b58adc30058be6fedbac290221f3b2b947946b06b25fe8783f697afc
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

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
