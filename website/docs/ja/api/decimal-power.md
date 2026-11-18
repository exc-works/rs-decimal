---
locale: ja
source_revision: 6c8f91b0409cc71714a4b9926acf34a3cf0aa635f8904f2e607ed3243fb8b30a
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::power
---
# Decimal::power

元のスケールで整数乗します。

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## 引数、戻り値、通常の使い方

power: i64 を bigint で渡します。入力のスケールへ最近接偶数丸めを行い、負の指数では逆数を取ります。2.00 の -3 乗は 0.12 ですが、整数スケールの 2 の -3 乗は 0 です。0 の 0 乗は初期化済みの 1、0 の負の累乗は DivideByZero です。コアの中間上限を超える巨大な指数は割り当て前に Overflow を返します。ブラウザーの中間スケール・係数の上限はさらに小さく、0 と +/-1 ではコアの高速経路を保持します。入力は借用します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
