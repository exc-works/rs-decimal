---
locale: ja
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
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

## 関連メソッド

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
