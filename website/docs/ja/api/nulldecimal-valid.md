---
locale: ja
source_revision: 2a36a5c46f57052b68644728359c44f15465f6bfca2c60d1540e40c603cc8c5b
translation_provenance: en-US-japanese-reviewed
api_id: NullDecimal.valid
---
# NullDecimal.valid

保存された小数を変えず、値の有無を読み取りまたは変更します。

```rust
valid: bool
```

## 通常の使い方と意味

この公開 Rust bool フィールドは boolean の getter/setter に対応します。false に設定しても小数を消去せず、true に設定しても初期化しません。JavaScript は boolean 以外の値を変更前に拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-valid" locale="ja" />

共通の例をネイティブ Rust とブラウザーで実行します。

## 関連ドキュメント

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
