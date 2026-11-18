---
locale: ja
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
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

## ブラウザーの制約

config と serde 機能は有効です。ソーステキスト／バイト列は UTF-8 で 8192 バイトまでです。Rust は解析前に小数テキストとデコード済み JSON 文字列を確認するため、エスケープで[リソースポリシー](../resource-policy)を回避できません。128 ビット整数には範囲内の bigint が必要です。借用する Decimal/NullDecimal ハンドルはデコード時に使用可能である必要があります。変換は入力を消費せず、独立した値を作ります。汎用 Serde トレイトの動作は具体的な JSON アダプターで示します。

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
