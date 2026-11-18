---
locale: zh-Hans
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
api_id: Decimal::string
---
# Decimal::string

格式化为定点十进制文本。

```rust
fn string (& self) -> String
```

## 参数、返回值及常规用法

无参数，返回 String/string，不使用科学记数法。移除多余的小数末尾零，保留整数末尾零。使用小数点，不分组。负零没有单独的表示。默认零输出 0，不因此被初始化；格式化仅借用输入。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
