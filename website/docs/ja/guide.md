---
locale: ja
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# はじめに

入力を正確に保持するには文字列から十進数を生成します。`mul_exact` は入力の小数点以下の桁数を合計します。`string()` は末尾のゼロを省略し、`string_with_trailing_zeros()` は保持します。

<<< ../../../examples/web_getting_started.rs

precision 引数は有効数字ではなく小数点以下の桁数です。ちょうど中間の場合、`HalfEven` は残す末尾の数字が偶数になる結果を選びます。`Down` はゼロ方向への切り捨てで、負の無限大方向ではありません。

`Decimal::default()` は未初期化の数値ゼロです。初期化済みのゼロと数値は等しくても状態が異なります。`NullDecimal.valid` は値の有無を独立して決めます。

失敗する可能性のある通常のメソッドは `Result` を返します。エラー種別を確認してください。想定される入力エラーで panic するのは明示的に `must_*` と名付けられたメソッドだけです。ブラウザーは JavaScript bridge を介して同じ Rust メソッドを呼び出し、型と所有権の対応を説明します。
