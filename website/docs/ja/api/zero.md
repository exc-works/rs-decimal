---
locale: ja
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
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

## 制約

bridge は Rust の名前と引数の順序を保持します。自由関数は playground で直接使用できます。借用入力を消費したり変更したりせず、不正なハンドルや別の実行のハンドルは拒否します。すべての操作は[リソースポリシー](../resource-policy)に従います。

## 関連メソッド

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
