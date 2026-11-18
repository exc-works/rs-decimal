---
locale: ja
source_revision: 78ca92acc2a8523b4e668da7c55a57a56b16410b6455b88e11d916edee01152c
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

設定入力 Null を選択します。

```rust
Null
```

## 引数、戻り値、通常の使い方

存在しない設定入力です。decode_nullable は無効な値を返し、decode_decimal は Unmarshal で拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
