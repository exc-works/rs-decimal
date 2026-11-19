---
locale: ja
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
translation_provenance: en-US-japanese-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

正確な JSON 設定フィールドをデシリアライズします。

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## 引数、戻り値、通常の使い方

deserializer: Serde Deserializer を実装する D。#[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")] 向けです。serde_json ソースと config 機能が必要です。浮動小数点の往復なしに元の数値トークンを保持します。ブラウザーは任意の deserializer ではなく、完全な JSON スカラーのソース文字列 1 つを受け取る具体的なアダプターを提供します。非 nullable の対象では JSON null と空文字列を拒否します。エラーは Serde deserializer のエラー型です。ブラウザーの category="serde" は DecimalError の種類を作らず、元のメッセージを保持します。末尾の JSON 空白以外の入力は拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
