---
locale: ja
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

整数部分とスケール適用前の小数部分の剰余に分けます。

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 引数、戻り値、通常の使い方

引数なし。(BigInt, BigInt) を返し、[bigint, bigint] に対応します。第 1 要素はゼロ方向に切り捨てた値、第 2 要素は符号付きの係数剰余であり、Decimal の小数部分ではありません。-12.750 の結果は (-12, -750) です。第 2 要素を -0.750 と解釈するには元のスケール 3 が必要です。既定のゼロは (0, 0) を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="ja" />

## ブラウザーの制約

上記の所有権規則に従ってください。ムーブ後のアクセスは moved エラーになり、free は冪等です。型とエイリアスの検証はムーブ前に行いますが、Rust のエラーは消費済みのレシーバーを復元しません。[リソースポリシー](../resource-policy)はライブラリエラーとは別です。累乗では中間値を、桁揃えでは係数の増大を確認します。保守的な検査により最終結果が小さくても拒否される場合があります。

## 関連メソッド

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
