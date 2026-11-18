---
locale: zh-Hans
source_revision: 4beaa8ccdf2751ba7bce74d134b8de29fabd5c0d82d0d4eebee1575a204be12d
api_id: reexport::GoFormat
---
# reexport::GoFormat

从 crate 根路径导入 GoFormat。

```rust
pub use formatting::GoFormat as GoFormat
```

## 参数、返回值及常规用法

这是 Rust 结构体的公开重新导出，不是另一份实现。浏览器提供字段对象适配器和调用 Rust 的默认值工厂。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-reexport-goformat" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
