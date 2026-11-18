---
locale: ja
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::neg
---
# Decimal::neg

符号を反転した独立した小数を作成します。

```rust
fn neg (& self) -> Self
```

## 引数、戻り値、使用上の注意

Decimal を返し、スケールと末尾のゼロを保持します。ゼロは数値としてゼロのままで、入力が既定値でも結果は初期化済みになります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="ja" />

## エラーと制約

これらのメソッドは入力を借用し、消費も変更もしません。ライブラリの操作は Result を返しません。bridge は解放済み、ムーブ済み、別の実行に属するハンドルを拒否し、[リソースポリシー](../resource-policy)を適用します。例ではスカラー出力を `value` フィールドに包み、Decimal 出力には完全なスナップショットを使います。

## 関連メソッド

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
