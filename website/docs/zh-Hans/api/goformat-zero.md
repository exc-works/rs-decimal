---
locale: zh-Hans
source_revision: 5fb7d2ba1d450e55c06f4bd243bcd776a0cb6c5e8f2e0f80f16235d6abf720a7
api_id: GoFormat.zero
---
# GoFormat.zero

设置 zero 格式字段。

```rust
zero: bool
```

## 参数、返回值及常规用法

在符号之后用零填充数字左侧。left 为 true 或 d 有显式 precision 时忽略。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
