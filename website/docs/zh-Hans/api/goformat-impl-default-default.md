---
locale: zh-Hans
source_revision: 3ce1c461a5149267a6d290d24a336549ce4f3b1fe8eb45cc970b59fa53adfd15
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

使用 Rust Default::default 实现。

```rust
fn default () -> Self
```

## 参数、返回值及常规用法

GoFormat 是小型 Copy 字段结构体。Clone/Copy 保留选项；Debug 输出字段。Default 选择 v，width/precision 为 None，所有标志为 false。此 Rust 语法在原生环境测试；bridge 对象使用普通 JS 字段对象语义及 GoFormat.default()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
