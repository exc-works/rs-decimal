---
locale: ja
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

正確な小数の規則を評価します。

```rust
fn matches (& self , value : & Decimal) -> bool
```

## 引数、戻り値、通常の使い方

value: &Decimal/Decimal。bool/boolean を返し、規則と値の両方を保持します。規則は浮動小数点変換なしに正確な Decimal 値を比較します。decimal_required は初期化状態を調べるため、初期化済みのゼロは通過します。eq/ne/gt/gte/lt/lte は未初期化のゼロも数値として比較します。between は両端を含む lower~upper を使い、初期化も要求します。positive/negative/nonzero も初期化を要求します。max_precision は末尾のゼロを含む保存スケールを調べ、required は暗黙に要求しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="ja" />

## ブラウザーの制約

validation 機能は有効です。各文字列は UTF-8 で 8192 バイト、規則／エラーのリストとメッセージマップは 64 項目、JSON 転送は 65536 バイトまでです。規則の小数引数は[リソースポリシー](../resource-policy)に従います。翻訳前に置換後テキストの保守的な合計上限 65536 バイトを検査し、実際にはちょうど収まる結果も拒否する場合があります。独自のタグとフィールドはデータであり、実行するコードではありません。検証失敗、設定エラー、bridge のリソースエラーは区別します。

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
