---
locale: ja
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
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

## 関連メソッド

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
