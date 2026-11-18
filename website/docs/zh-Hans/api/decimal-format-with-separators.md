---
locale: zh-Hans
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

对整数位分组，并选择小数分隔符。

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 参数、返回值及常规用法

thousands: char；decimal: char。返回 String/string，整数位每三位一组，保留所有存储的小数位。每个 JS 参数必须恰好是一个 Unicode 标量，可使用补充平面字符；孤立代理项和多字符字符串被拒绝。NUL thousands 禁用分组，NUL decimal 使用小数点。分隔符是字面字符，不是语言区域名称。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="zh-Hans" />

## 浏览器边界

遵循 [资源限制](../resource-policy)：格式宽度最多 65536 字节，precision 最多 1000，固定格式还检查系数增长。缓冲区总长最多 65536 字节，追加前使用保守长度估计，因此可能拒绝实际可容纳的临界操作。资源错误不会改变输入或缓冲区。print 对长文本另有显示上限；需要检查完整输出时可读取 length 或分段打印。

## 相关方法

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
