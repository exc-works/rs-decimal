---
locale: zh-Hans
source_revision: 2264e63e21d4ba0bb0732ee23526e08fc191d66bd401bfd2f899bab466b9c36e
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

直接解析精确十进制文本，不经过二进制浮点数。通常用于用户输入的小数金额。

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## 参数和返回值

`text: &str` 对应 JavaScript `string`。Rust 返回 `Result<Decimal, DecimalError>`；bridge 返回 Decimal 或抛出保留库分类的错误。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="zh-Hans" />

## 使用说明

输入会去掉首尾空白，可包含符号和指数。小数点两边都必须有数字，分隔符始终是 `.`，不随界面语言改变。非零值保留尾零，但 `"0.00"` 会规范化为已初始化的 scale 为零的零。错误语法返回 `InvalidFormat`；指数展开后可能触及独立的 Playground 资源限制。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
