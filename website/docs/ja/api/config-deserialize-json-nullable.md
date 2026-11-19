---
locale: ja
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
translation_provenance: en-US-japanese-reviewed
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

正確な JSON 設定フィールドをデシリアライズします。

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## 引数、戻り値、通常の使い方

deserializer: Serde Deserializer を実装する D。#[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")] 向けです。serde_json ソースと config 機能が必要です。浮動小数点の往復なしに元の数値トークンを保持します。ブラウザーは任意の deserializer ではなく、完全な JSON スカラーのソース文字列 1 つを受け取る具体的なアダプターを提供します。JSON null と厳密に空の JSON 文字列は無効になります。空白だけの文字列は失敗します。空でない値が成功すると有効になります。エラーは Serde deserializer のエラー型です。ブラウザーの category="serde" は DecimalError の種類を作らず、元のメッセージを保持します。末尾の JSON 空白以外の入力は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
