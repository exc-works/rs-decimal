---
locale: zh-Hans
source_revision: 1bcdb4a809c3db807556fea8772e6afac8daed8fef0faa997fbb848ae1a34a13
api_id: GoFormat.left
---
# GoFormat.left

设置 left 格式字段。

```rust
left: bool
```

## 参数、返回值及常规用法

在右侧填充空格。左对齐优先于数字零填充。默认为 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
