---
locale: ja
source_revision: 4748c543f00fa16cc184b2f3ac3218ce12c09982f49453b64f19d87393fb722a
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

設定入力 Nullable を選択します。

```rust
Nullable (& 'a NullDecimal)
```

## 引数、戻り値、通常の使い方

既存の nullable を借用します。decode_nullable は有効性と隠れたデータを複製します。decode_decimal は有効な値のみを受け取り、その内部 Decimal を複製します。デコード前の変更は反映されます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
