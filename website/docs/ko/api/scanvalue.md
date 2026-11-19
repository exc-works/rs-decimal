---
locale: ko
source_revision: 1d04e1207bda167aea5c52bc9b525b3aa5208f7c0d21a8e9e4cdf61685f69f69
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue"
---
# ScanValue

드라이버에 독립적인 타입 지정 SQL 입력을 선택합니다.

```rust
pub enum ScanValue< 'a >
```

## 매개변수, 반환값, 일반적인 사용법

이 열거형은 NULL, f32, f64, i64, 텍스트, 텍스트 바이트, 미지원 값을 구분합니다. Decimal.scan 또는 NullDecimal.scan 전에 명시적인 변형을 선택하세요. Rust에서는 빌린 텍스트/바이트가 호출 중에만 유효하면 됩니다. 브라우저 Bytes는 호출자 배열을 유지하며 scan 전에 한 수정을 반영합니다. 데이터베이스 서비스에 접속하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
