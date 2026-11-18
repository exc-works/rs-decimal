---
locale: ja
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
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

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
