---
locale: ja
source_revision: 74e7f2ffd43f198c10e7ebd972920c487311d6c7ab45db8c6281425ea90f9c14
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

設定入力 Number を選択します。

```rust
Number (& 'a str)
```

## 引数、戻り値、通常の使い方

浮動小数点へ変換せず、数値のソーステキストを保持します。コアは小数構文を解析します。このバリアントは完全な JSON 文法検証器ではありません。空の Number は nullable の対象でも不正です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="ja" />

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
