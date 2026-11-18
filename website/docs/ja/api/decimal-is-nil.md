---
locale: ja
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

小数が未初期化の既定値かどうか調べます。

```rust
fn is_nil (& self) -> bool
```

## 引数、戻り値、使用上の注意

bool/boolean を返します。既定値は nil、構築したゼロは初期化済みですが、両者は数値として等価です。これは NullDecimal の有効性フラグではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
