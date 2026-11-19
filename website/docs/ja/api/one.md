---
locale: ja
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
translation_provenance: en-US-japanese-reviewed
api_id: one
---
# one

十進整数 1 を構築します。

```rust
fn one () -> Decimal
```

## 引数、戻り値、使用上の注意

引数なし。スケール 0 の新しい初期化済み Decimal を返します。ある結果の変更や解放は、後の呼び出しに影響しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="ja" />

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
