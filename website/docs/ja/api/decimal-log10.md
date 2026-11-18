---
locale: ja
source_revision: 451ec0bab626d0ba18b68c82b3fd8391b7b06c98aa89fa00c9f2a7af5fce4bb4
translation_provenance: en-US-japanese-reviewed
api_id: Decimal::log10
---
# Decimal::log10

底 10 の対数を計算します。Rust は Result<Decimal, DecimalError>、bridge は Decimal または構造化された例外を返します。引数なし。既定の出力精度は max(入力の小数点以下の桁数, 20) であり、有効数字 20 桁ではありません。入力は正である必要があり、ゼロや負の入力は InvalidLog を返します。底の正確な整数乗の対数は正確な整数となり、出力スケールで保持します。コアは値を囲む区間を使い、丸めを証明できたときだけ結果を受理します。mode 引数のない最近接偶数丸めです。コアの上限内で証明できない場合は、未検証の近似値ではなく NoConvergence を返します。入力は変わらず、成功した結果は初期化済みです。

```rust
fn log10 (& self) -> Result < Self , DecimalError >
```

## 通常の使い方

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10" locale="ja" />

## ブラウザーの制約

このグループには追加の[リソース制限](../resource-policy)があります。出力精度は最大 128、入力スケールは最大 256、入力係数は最大 4096 ビットです。通常の exp 計算では入力の絶対値も最大 64 です。コアが直接ゼロに丸められると証明できる負の入力は高速経路を保持します。入力スケールが大きいと、引数なしのメソッドが 128 を超える既定精度を選ぶ場合があります。with_prec で許可された出力精度を明示してください。リソースエラーとコアの NoConvergence は区別します。Stop は実行を中断し、次の実行では新しい Wasm インスタンスを作ります。

## 関連メソッド

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
