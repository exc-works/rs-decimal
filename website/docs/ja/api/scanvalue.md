---
locale: ja
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
translation_provenance: en-US-japanese-reviewed
api_id: "ScanValue"
---
# ScanValue

ドライバーに依存しない型付き SQL 入力を選択します。

```rust
pub enum ScanValue< 'a >
```

## 引数、戻り値、通常の使い方

この列挙型は NULL、f32、f64、i64、テキスト、テキストのバイト列、未対応の値を区別します。Decimal.scan または NullDecimal.scan の前にバリアントを明示的に選びます。Rust では借用テキスト／バイト列は呼び出し中に有効なら十分です。ブラウザーの Bytes は呼び出し側の配列を保持し、scan 前の変更を反映します。データベースサービスへの接続は行いません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="ja" />

## ブラウザーの制約

SQL のテキスト／バイト入力とエラーメッセージは UTF-8 で 8192 バイトまでです。SQL テキストは外側の引用符を除去した後で[リソース検査](../resource-policy)を受けるため、引用符で指数上限を回避できません。借用呼び出しは入力を保持し、消費済みや解放済みのハンドルは再利用できません。Rust のトレイト／インポート例はネイティブテストで実行し、ブラウザーのメソッド対応を意味しません。

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
