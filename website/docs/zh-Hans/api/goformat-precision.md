---
locale: zh-Hans
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
api_id: GoFormat.precision
---
# GoFormat.precision

设置 precision 格式字段。

```rust
precision: Option < usize >
```

## 参数、返回值及常规用法

精度含义随动词变化：f/F 为小数位，e/E 为首位之后的位数，g/G 为有效数字（零按一处理），d 为最少整数位。None/null 使用动词默认规则；f 保留 scale，e 使用六位。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
