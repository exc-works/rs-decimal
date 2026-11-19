---
locale: ja
source_revision: 6fa89e7d9fd5a43bce6d83be2495a38047592a77e9611dcd781da000b62e4862
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::unmarshal_text"
---
# Decimal::unmarshal_text

text 入力を既存の Decimal へデコードします。

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

bytes: &[u8]/Uint8Array。レシーバーを変更し、Result<(), DecimalError>/undefined を返します。前後の空白除去と指数構文を含む小数テキストを解析します。空テキストとリテラル null は不正な小数テキストです。空が値なしを意味する場合は nullable のデコードを使います。null 以外の入力の解析に成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持し、解析したゼロはスケール 0 に正規化します。エラー時はレシーバーの全状態を保持します。ライブラリのデコードエラーは Unmarshal と基底の種類を保持し、リソースの失敗も変更前に起こります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-text" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
