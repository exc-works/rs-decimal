---
locale: zh-Hans
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
api_id: Decimal::neg
---
# Decimal::neg

创建符号相反的独立 Decimal。

```rust
fn neg (& self) -> Self
```

## 参数、返回值及注意事项

返回 Decimal，保留小数位数及末尾零。零在数值上仍为零；即使输入为默认值，结果也会初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
