---
locale: zh-Hant
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::neg
---
# Decimal::neg

建立符號相反的獨立 Decimal。

```rust
fn neg (& self) -> Self
```

## 參數、回傳值及注意事項

回傳 Decimal，保留小數位數及末尾零。零在數值上仍為零；即使輸入為預設值，結果也會初始化。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
