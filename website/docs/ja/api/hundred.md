---
locale: ja
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
translation_provenance: en-US-japanese-reviewed
api_id: hundred
---
# hundred

十進整数 100 を構築します。

```rust
fn hundred () -> Decimal
```

## 引数、戻り値、使用上の注意

引数なし。スケール 0 の新しい初期化済み Decimal を返します。値を構築する関数であり、共有される可変定数ではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="ja" />

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
