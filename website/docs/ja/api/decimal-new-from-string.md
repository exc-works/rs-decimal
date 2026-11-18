---
locale: ja
source_revision: 3ef0ab66b1050d81978ffe9504c298d164d62d052b64bfd1ad02ddce4a039f36
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::new_from_string
---
# Decimal::new_from_string

二進浮動小数点を経由せず、正確な小数テキストを解析します。ユーザーが入力した小数の金額には通常このコンストラクターを使います。

```rust
fn new_from_string (text : & str) -> Result < Self , DecimalError >
```

## 引数と戻り値

`text: &str` → JavaScript の `string`。Rust は `Result<Decimal, DecimalError>` を返し、bridge は Decimal を返すか、同じライブラリのエラー種別を送出します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-string" locale="ja" />

## 使用上の注意と制約

入力の前後の空白は除去され、符号と指数を含められます。小数点の両側には数字が必要です。区切りは UI 言語に関係なく常に `.` です。ゼロ以外の値では末尾のゼロを保持しますが、`"0.00"` はスケール 0 の初期化済みゼロに正規化します。不正な構文は `InvalidFormat` です。展開後の入力は別途 playground のリソース制限に達することがあります。

ブラウザーは[リソースポリシー](../resource-policy)も適用します。JavaScript 入力は Rust の呼び出し前に検証します。

## 関連メソッド

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
