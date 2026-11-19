---
locale: zh-Hans
source_revision: eb4bab6a9fdfe771d31eefc8731c88dad81f2f8a3273adea4040fea11f4141ab
api_id: Decimal::sign
---
# Decimal::sign

以 -1、0、1 读取符号。

```rust
fn sign (& self) -> i32
```

## 参数、返回值及注意事项

返回 i32/number。已初始化和未初始化的零都返回 0，不保留单独的负零符号。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
