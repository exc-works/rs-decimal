---
locale: ja
source_revision: 906bb0bd7555c68ed2da4c7be205d647a1ca94b724e2e70f5d1160d968357278
translation_provenance: en-US-japanese-reviewed
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

正確な小数の規則を評価します。

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

tag: &str/string、parameter: &str/string。`Result<ValidationRule, DecimalError>`/規則ハンドルを返します。decimal_gt のような完全なタグを使います。比較引数は小数テキスト、between は lower~upper、max_precision は非負の isize を取ります。不明なタグ、不正な引数、逆転した範囲は InvalidArgument です。wasm32 の isize は符号付き 32 ビットです。引数不要の規則は引数テキストを保持しますが解釈しません。規則は浮動小数点変換なしに正確な Decimal 値を比較します。decimal_required は初期化状態を調べるため、初期化済みのゼロは通過します。eq/ne/gt/gte/lt/lte は未初期化のゼロも数値として比較します。between は両端を含む lower~upper を使い、初期化も要求します。positive/negative/nonzero も初期化を要求します。max_precision は末尾のゼロを含む保存スケールを調べ、required は暗黙に要求しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="ja" />

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
