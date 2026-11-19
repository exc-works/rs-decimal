---
locale: ja
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
translation_provenance: en-US-japanese-reviewed
api_id: zero
---
# zero

スケール 0 の初期化済みゼロを構築します。

```rust
fn zero () -> Decimal
```

## 引数、戻り値、使用上の注意

引数なし。新しい Decimal を返します。Decimal::default() とは異なり結果は初期化済みですが、数値としては等しい値です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="ja" />

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
