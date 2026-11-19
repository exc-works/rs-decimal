---
locale: ja
source_revision: 967fc6e56bfb3ade48f0faf10b0c1e0de7e21a4d9ec093d537fb469385460f6f
translation_provenance: en-US-japanese-reviewed
---
# ブラウザーから Rust メソッドを呼び出す

playground は、Wasm にコンパイルされた Rust ライブラリを呼び出す JavaScript を実行します。
メソッド名、引数の順序、丸め、小数の状態は Rust に従います。JavaScript は制御フローを提供し、小数の結果を計算しません。個別の動作は[メソッド一覧](./api/)、実行は [Playground](./playground) を参照してください。

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

小数入力には正確な文字列、i64 引数には `bigint` を使います（`3` ではなく `3n`）。
`Decimal.new` は i64 範囲外の値、数値文字列、JS number を拒否します。精度は wasm32 の符号付き範囲内の JS 整数です。小数、非有限値、範囲外の値は、ABI キャストで折り返しや切り捨てが起こる前に拒否します。負の整数精度は Rust まで渡され、元の InvalidPrecision エラーを返します。

## エラーと所有権

例外の `kinds` 配列で `InvalidPrecision` などの計算エラーを判別できます。引数の型や範囲に問題がある場合、エラーの分類は `parameter` になります。

`rescale` を呼び出した後も元の値を使えます。`into_rescaled` は丸めに失敗した場合も元の値を消費します。元の値を残したい場合は先に `clone` を呼び出してください。

## Nullable 値とバイト列

`NullDecimal.new(decimal)` は引数を消費し、未初期化のゼロでも `valid=true` にします。`NullDecimal.default()` は値なしです。`valid` の設定は有効性だけを変え、内部の値を消しません。`get_decimal_clone()` は独立したコピーを返す明示的なアダプターです。`set_decimal(value)` は有効性を変えずに、新しい内部値をムーブします。変更を追跡する参照と誤解されるような `.decimal` getter はありません。

同様に `coefficient()` は不変の JS bigint スナップショットを返します。JavaScript は Rust の借用ライフタイムやコンパイル時のトレイト検査を再現しません。`ByteBuffer.new(Uint8Array)` はバイト列を所有するバッファーへコピーし、`to_uint8_array()` は Wasm メモリーが拡張されても有効なコピーを返します。追記メソッドは[メソッド一覧](./api/)で説明します。

JavaScript の `snapshot(value)` は状態を即座に正確に取得し、無効な nullable の隠れた値も含みます。返すデータは凍結されているため、後の変更が過去の出力を変えることはありません。隔離された runner は再帰的な出力と実行時間を制限します。[リソースポリシー](./resource-policy)を参照してください。
