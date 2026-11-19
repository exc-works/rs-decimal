---
locale: ko
source_revision: 17f954a2ac80c90ce4a20dd8e36b0981b73e6748171ba829abe55cdfdc69b2b3
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Null"
---
# ScanValue::Null

SQL 입력 변형 Null을 선택합니다.

```rust
Null
```

## 매개변수, 반환값, 일반적인 사용법

SQL NULL을 나타냅니다. scan은 Decimal을 미초기화 0으로 재설정하고 NullDecimal을 비웁니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-null" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
