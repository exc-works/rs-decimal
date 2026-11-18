---
locale: ja
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::precision
---
# Decimal::precision

値に保存された小数点以下の桁数を読み取ります。

```rust
fn precision (& self) -> isize
```

## 引数、戻り値、使用上の注意

isize（ブラウザーでは整数の number）を返します。有効数字の数ではなくスケールであり、末尾のゼロも数えます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
