---
locale: ja
source_revision: 4f9cc4e1f27490d354a3bbc2ca44814e0fd0d846ebf81964d518e23cc81067c0
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

ゼロ方向に切り捨てる除算の符号付き剰余を返します。

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

other: &Decimal。Rust は Result<Decimal, DecimalError> を返します。剰余は被除数の符号と両オペランドの大きい方のスケールを持ち、非負のユークリッド剰余ではありません。除数がゼロなら DivideByZero です。入力は借用します。商と剰余の両方が必要なら quo_rem を使います。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
