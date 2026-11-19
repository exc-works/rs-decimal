---
locale: ja
source_revision: 37b57334dcd3fd469d86572a66aa34315b91919270af861524b7fae399085f7c
translation_provenance: en-US-japanese-reviewed
api_id: ten
---
# ten

十進整数 10 を構築します。

```rust
fn ten () -> Decimal
```

## 引数、戻り値、使用上の注意

引数なし。スケール 0 の新しい初期化済み Decimal を返します。小数点以下の桁が必要なら rescale を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="ja" />

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
