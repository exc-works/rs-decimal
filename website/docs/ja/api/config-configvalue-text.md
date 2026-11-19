---
locale: ja
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

設定入力 Text を選択します。

```rust
Text (& 'a str)
```

## 引数、戻り値、通常の使い方

正確な小数テキストを借用します。前後の空白は小数コアが処理します。SQL 形式の外側の引用符は除去しません。厳密に空のテキストだけが nullable の値なしを意味します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
