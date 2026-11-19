---
locale: ja
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

スケール適用前の正確な整数係数を確認します。

```rust
fn coefficient (& self) -> & BigInt
```

## 引数、戻り値、使用上の注意

Rust は &BigInt を借用します。JavaScript は不変の bigint スナップショットを返し、変更を追跡する参照ではありません。係数には末尾のゼロが含まれ、小数の整数部分とは異なります。例では JSON の全桁を保つため bigint をテキストとして表示します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="ja" />

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
