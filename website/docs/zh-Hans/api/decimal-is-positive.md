---
locale: zh-Hans
source_revision: 2ea2866a9b7493c17d529f3372e5bbb238200663f6cd52e7c63d668cc43d0ffe
api_id: Decimal::is_positive
---
# Decimal::is_positive

判断数值是否严格大于零。

```rust
fn is_positive (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean。零（包括未初始化的默认值）返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
