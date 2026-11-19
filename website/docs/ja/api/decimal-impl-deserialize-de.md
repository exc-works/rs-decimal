---
locale: ja
source_revision: 290824d81667633f4375180726adda2c851b5415686c46a9a13af6b626baaf4f
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Deserialize < 'de >"
---
# Decimal::impl Deserialize < 'de >

汎用 Serde visitor でデシリアライズします。

```rust
impl< 'de > Deserialize < 'de > for Decimal
```

## 引数、戻り値、通常の使い方

deserializer: Deserializer を実装する D。Self または D::Error を返します。正確な小数文字列、整数 visitor 入力、null を受け取ります。浮動小数点 visitor 入力は拒否します。小数や指数を含む JSON 数値には専用の生 JSON/config アダプターを使う必要があります。JSON null は未初期化の Decimal ゼロになります。空文字列は不正です。ブラウザーの静的 deserialize(source) アダプターは元の完全な JSON テキストに serde_json を実行し、任意の Rust deserializer の対応範囲より狭くなります。エラーは category="serde" と元のメッセージを公開します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
