---
locale: zh-Hant
source_revision: 3c6ebaee1d6c2d15a7801caee0a8597128eea9b110eb2748ac00d60cf69e23fd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::scan"
---
# NullDecimal::scan

讀取與資料庫驅動無關的 SQL 純量。

```rust
fn scan (& mut self , value : ScanValue < '_ >) -> Result < () , DecimalError >
```

## 參數、回傳值及常規用法

value: ScanValue；修改接收者，回傳 `Result<(), DecimalError>`/undefined。明確選擇變體：Int64 使用精確 bigint，Float32 收窄為 binary32，Text/Bytes 保留原始小數數字。SQL 文字可帶一對首尾雙引號，只移除引號而不進行 JSON 反跳脫。NULL 清除有效性旗標並將內部 Decimal 重置為未初始化零；非空輸入成功後設定 valid=true。不支援的類型和非有限浮點數回傳 Unmarshal，其他解碼錯誤保留底層類別。所有錯誤均保留接收者狀態。此適配器不需要資料庫連線。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-scan" locale="zh-Hant" />

## 相關方法

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
