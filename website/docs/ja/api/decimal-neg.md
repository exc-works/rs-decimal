---
locale: ja
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::neg
---
# Decimal::neg

符号を反転した独立した小数を作成します。

```rust
fn neg (& self) -> Self
```

## 引数、戻り値、使用上の注意

Decimal を返し、スケールと末尾のゼロを保持します。ゼロは数値としてゼロのままで、入力が既定値でも結果は初期化済みになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
