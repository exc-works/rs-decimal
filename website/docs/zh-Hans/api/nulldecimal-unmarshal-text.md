---
locale: zh-Hans
source_revision: b6885bfd89f5c1e43b9c4a8ab207ccde2aef139d913f8cd028eda20cd2081e75
api_id: "NullDecimal::unmarshal_text"
---
# NullDecimal::unmarshal_text

将 text 输入解码到现有 NullDecimal。

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

bytes: &[u8]/Uint8Array；修改接收者，返回 Result<(), DecimalError>/undefined。解析十进制文本，包括首尾空白和指数语法。空输入清除有效标志并重置内部 Decimal；字面 null 不是合法小数文本。成功解析非空值会初始化小数，保留非零值的末尾精度，并将解析得到的零规范化为 scale 零。同时设置 valid=true。错误保留接收者的完整状态。库解码错误保留 Unmarshal 及底层类别；资源错误同样发生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-text" locale="zh-Hans" />

## 浏览器边界

解码输入最多 8192 字节，参数字符串还在复制到 Wasm 前检查长度。Rust 对文本及 JSON 解码后的字符串再次检查 [十进制资源限制](../resource-policy)，因此转义字符不能绕过指数展开或精度限制。无效 UTF-8 和 JSON/小数语法仍由核心报告。输入错误和资源错误都不会部分更新接收者；已移动的对象不可再读写。

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
