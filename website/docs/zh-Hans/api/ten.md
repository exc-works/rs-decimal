---
locale: zh-Hans
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
api_id: ten
---
# ten

构造值为十的 Decimal 整数。

```rust
fn ten () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的已初始化 Decimal，小数位数为零。需要小数位数时使用 rescale。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="zh-Hans" />

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
