---
locale: zh-Hans
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
api_id: GoFormat
---
# GoFormat

使用字段对象描述一次格式化操作。

```rust
pub struct GoFormat
```

## 参数、返回值及常规用法

字段对应 Rust 结构体：verb、width、precision、left、plus、space、zero、alternate。bridge 的 GoFormat.default() 从 Rust 读取默认值，返回新的可变对象；修改它不会设置全局格式。width/precision 使用 null 表示 None，非负 wasm32 整数 number 表示 Some。Rust trait 仍属于原生语法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
