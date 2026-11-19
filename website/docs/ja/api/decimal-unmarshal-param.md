---
locale: ja
source_revision: c4d07db94cb802a1abe829e9593804f52c2de0bb0ad545606bf47e6e769fde05
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::unmarshal_param"
---
# Decimal::unmarshal_param

param 入力を既存の Decimal へデコードします。

```rust
fn unmarshal_param (& mut self , param : & str) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

param: &str/string。レシーバーを変更し、Result<(), DecimalError>/undefined を返します。前後の空白除去と指数構文を含む小数テキストを解析します。空テキストとリテラル null は不正な小数テキストです。空が値なしを意味する場合は nullable のデコードを使います。null 以外の入力の解析に成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持し、解析したゼロはスケール 0 に正規化します。エラー時はレシーバーの全状態を保持します。ライブラリのデコードエラーは Unmarshal と基底の種類を保持し、リソースの失敗も変更前に起こります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-unmarshal-param" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
