---
locale: ja
source_revision: 949ffe2e68cd75d5b05be8db37757052323a51952d74093deec642c235953650
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_text"
---
# NullDecimal::unmarshal_text

text 入力を既存の NullDecimal へデコードします。

```rust
fn unmarshal_text (& mut self , text : & [u8]) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

bytes: &[u8]/Uint8Array。レシーバーを変更し、Result<(), DecimalError>/undefined を返します。前後の空白除去と指数構文を含む小数テキストを解析します。空入力は有効性を解除し、内部 Decimal をリセットします。リテラル null は不正な小数テキストです。null 以外の入力の解析に成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持し、解析したゼロはスケール 0 に正規化します。valid=true にします。エラー時はレシーバーの全状態を保持します。ライブラリのデコードエラーは Unmarshal と基底の種類を保持し、リソースの失敗も変更前に起こります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-unmarshal-text" locale="ja" />

## 関連メソッド

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
