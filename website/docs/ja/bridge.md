---
locale: ja
source_revision: bc4fe0d7f63ebbaad6f009ea77ae93eee10d61514aa0fcf8907740ea4397dc70
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

Rust が返すエラーは、`name: 'DecimalError'`、`category: 'library'`、元の `message`、文字列配列 `kinds` を持つ Error として送出されます。デコーダーのエラーは複数の種類に一致する場合があるため、配列全体を保持してください。JS の型や範囲の失敗は `name: 'BridgeError'`、`category: 'parameter'` であり、Rust の算術エラーではありません。不正な数値の丸め判別値は Rust の検証に渡されます。

借用メソッドの呼び出し後もオブジェクトは使用できます。`into_rescaled` は Rust がエラーを返す場合もレシーバーを消費します。消費済みまたは解放済みの値を使うと `moved` BridgeError になります。不正な JS 引数の型は呼び出し前に拒否され、レシーバーを消費しません。`clone()` は表現と初期化状態を保持します。`free()` は使用中のハンドルを解放し、消費後に呼んでも問題ありません。runner はファイナライザーに依存せず、完了時に Wasm インスタンスを破棄します。別の bridge インスタンスのオブジェクトを呼び出しに渡すことはできません。

## Nullable 値とバイト列

`NullDecimal.new(decimal)` は引数を消費し、未初期化のゼロでも `valid=true` にします。`NullDecimal.default()` は値なしです。`valid` の設定は有効性だけを変え、内部の値を消しません。`get_decimal_clone()` は独立したコピーを返す明示的なアダプターです。`set_decimal(value)` は有効性を変えずに、新しい内部値をムーブします。変更を追跡する参照と誤解されるような `.decimal` getter はありません。

同様に `coefficient()` は不変の JS bigint スナップショットを返します。JavaScript は Rust の借用ライフタイムやコンパイル時のトレイト検査を再現しません。`ByteBuffer.new(Uint8Array)` はバイト列を所有するバッファーへコピーし、`to_uint8_array()` は Wasm メモリーが拡張されても有効なコピーを返します。追記メソッドは[メソッド一覧](./api/)で説明します。

bridge の `snapshot(value)` は状態を即座に正確に取得し、無効な nullable の隠れた値も含みます。返すデータは凍結されているため、後の変更が過去の出力を変えることはありません。隔離された runner は再帰的な出力と実行時間を制限します。[リソースポリシー](./resource-policy)を参照してください。
