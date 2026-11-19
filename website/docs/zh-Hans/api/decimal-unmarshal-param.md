---
locale: zh-Hans
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

将 param 输入解码到现有 Decimal。

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 参数、返回值及常规用法

param: &str/string；修改接收者，返回 Result<(), DecimalError>/undefined。解析十进制文本，包括首尾空白和指数语法。空文本和字面 null 都不是合法小数文本；空值表示无值时使用 nullable 解码。成功解析非空值会初始化小数，保留非零值的末尾精度，并将解析得到的零规范化为 scale 零。错误保留接收者的完整状态。库解码错误保留 Unmarshal 及底层类别；资源错误同样发生在修改之前。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="zh-Hans" />

## 相关方法

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
