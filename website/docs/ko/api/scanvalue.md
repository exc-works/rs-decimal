---
locale: ko
source_revision: bc03546de8ea5c2e53e57f5ce62fd19b75d7cd5d6737f4aa1452dbcef13f206d
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

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
