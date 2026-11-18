---
locale: ja
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

設定入力 Bytes を選択します。

```rust
Bytes (& 'a [u8])
```

## 引数、戻り値、通常の使い方

小数のバイナリペイロードではなく、小数テキストを含むバイト列を借用します。ブラウザーではデコード前の Uint8Array の編集を反映し、デコード自体は配列を変更しません。不正な UTF-8 は小数解析に失敗します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
