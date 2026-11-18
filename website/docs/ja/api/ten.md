---
locale: ja
source_revision: df5bdc8fba1feff0933958fdceab2cd0bf8d1a541a0f2e47f17bffe6a6338d47
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

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
