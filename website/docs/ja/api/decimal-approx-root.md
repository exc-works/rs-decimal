---
locale: ja
source_revision: a269819e7b61d5a164cb35c2c28efef7424a30b1113a1fd7abc0f0da7fe8a5f9
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::approx_root
---
# Decimal::approx_root

正の次数の累乗根を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。degree: i64 を bigint で渡します。次数は正である必要があり、それ以外は InvalidRoot です。奇数次は負の入力を許可し、負の値の偶数次根は NegativeRoot です。次数 1 は最近接偶数丸めによるスケール変更です。既定精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。丸めは最近接偶数丸めです。精度 0 の sqrt(6.25) のように正確に中間になる根は 2 に丸めます。approx_root という名前でも、Rust は整数比較で丸めを証明します。コアの計算量や反復上限に達すると NoConvergence を返します。入力は借用して変更せず、既定のゼロを含む結果は初期化済みです。

```rust
fn approx_root (& self , degree : i64) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-approx-root" locale="ja" />

## ブラウザーの制約

[リソースポリシー](../resource-policy)は出力スケールを 1000 に制限します。非自明な累乗根の次数は最大 128 で、スケール調整、Newton 法の累乗、中間点の証明の中間値を事前検査します。ゼロ、符号付き単位値、次数 1 の根はコアの高速経路を保持しますが、出力精度の制限には従います。定義域と負の精度のエラーはライブラリの動作を保持し、リソースエラーは NoConvergence と区別します。各入力が有効でも、すべての次数と精度の組み合わせがブラウザーの上限内とは限りません。

## 関連メソッド

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
