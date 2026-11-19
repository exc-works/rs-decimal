---
locale: zh-Hans
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
api_id: hundred
---
# hundred

构造值为一百的 Decimal 整数。

```rust
fn hundred () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的已初始化 Decimal，小数位数为零。它是数值构造函数，不是共享的可变常量。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="zh-Hans" />

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
