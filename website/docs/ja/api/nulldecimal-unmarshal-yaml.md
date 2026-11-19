---
locale: ja
source_revision: dbc15c20de89c4c0e7f4a73778b0f1b410d8c87e2c8de57f1e13798c66980cb2
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_yaml"
---
# NullDecimal::unmarshal_yaml

yaml を既存の値へデコードします。

```rust
fn unmarshal_yaml (& mut self , source : & str) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

source: &str/string。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。Rust は yaml を必要とします。Rust が引用符、タグ、正確な桁を扱えるよう、元の YAML ソースを渡します。引用符のない 012 は 8 進数として 10 になり、引用符付きの "012" または !!str 012 は十進数の 12 として解析します。数値タグ、基数、アンダースコアはコアの解決規則に従い、引用符なしの 1_000 は 1000、引用符付きの "1_000" は不正です。真偽値、コレクション、ブロックスカラーは拒否します。値は最初の文書だけから取得しますが、後続の文書も構文が有効である必要があります。Null または空の文書は valid を解除し、内部 Decimal を未初期化のゼロにリセットします。null 以外の入力のデコードが成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持します。valid=true にします。エラー時はレシーバーの全フィールドを保持し、Unmarshal を含むライブラリのエラー種別を保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-yaml" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
