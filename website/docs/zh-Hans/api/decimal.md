---
locale: zh-Hans
source_revision: 9459cb9d77c7f6ad22b96853baef01b1a9461529cecff18a116b5d2dee9166d3
api_id: Decimal
---
# Decimal

保存精确整数系数及非负的小数位数。

```rust
pub struct Decimal
```

## 常规用法及语义

数值为 coefficient × 10^-scale。scale 表示小数位数，包括保留的末尾零。默认值是未初始化的数值零。构造函数得到已初始化数值；借用方法保留输入，消耗方法会使浏览器包装对象失效。bridge 使用具名构造函数，new Decimal() 不是公开构造方式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal" locale="zh-Hans" />

这份共用示例同时在原生 Rust 和浏览器中执行。

## 相关文档

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
