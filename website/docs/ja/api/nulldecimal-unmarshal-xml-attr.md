---
locale: ja
source_revision: f9048a3e7634cd109eea9581928de5e077f7c7324b6f3b621f732137009253fb
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::unmarshal_xml_attr"
---
# NullDecimal::unmarshal_xml_attr

xml_attr を既存の値へデコードします。

```rust
fn unmarshal_xml_attr (& mut self , value : & str) -> Result < () , DecimalError >
```

## 引数、戻り値、通常の使い方

value: &str/string。レシーバーを変更し、`Result<(), DecimalError>`/undefined を返します。Rust は xml を必要とします。XML 属性の断片ではなく、実体をすでにデコードした属性値を渡してください。"1.23" は有効ですが、`"&#49;.23"` は再デコードされません。前後の空白を除去し、正確な小数テキストを解析します。内容が空なら valid を解除し、内部 Decimal を未初期化のゼロにリセットします。null 以外の入力のデコードが成功すると小数を初期化し、ゼロ以外の数値の末尾のスケールを保持します。valid=true にします。エラー時はレシーバーの全フィールドを保持し、Unmarshal を含むライブラリのエラー種別を保持します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="yaml-xml-nulldecimal-unmarshal-xml-attr" locale="ja" />

## 関連メソッド

[unmarshal_yaml](./decimal-unmarshal-yaml) · [unmarshal_xml](./decimal-unmarshal-xml) · [unmarshal_xml_attr](./decimal-unmarshal-xml-attr)
