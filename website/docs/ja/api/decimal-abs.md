---
locale: ja
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::abs
---
# Decimal::abs

絶対値を持つ独立した小数を作成します。

```rust
fn abs (& self) -> Self
```

## 引数、戻り値、使用上の注意

Decimal を返し、スケールと末尾のゼロを保持します。入力が既定値でも結果は初期化済みになり、入力は変更しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
