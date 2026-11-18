---
locale: ja
source_revision: 939b86ba367c0b35cb69b9f2664eb74fafe4279f8b1b81c20315cc4e07b18382
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

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
