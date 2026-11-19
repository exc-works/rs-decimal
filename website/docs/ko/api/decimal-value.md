---
locale: ko
source_revision: 8064cdb13ca114f6f1b29d727ed53f33f638f79926b35e606248ced643f822e9
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::value"
---
# Decimal::value

정규 SQL 값을 얻습니다.

```rust
fn value (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. 미초기화 Decimal도 NULL이 아닌 "0"을 반환하므로 SQL NULL에는 NullDecimal을 사용하세요. 소수 자릿수나 초기화를 바꾸지 않고 소수 부분의 후행 0을 제거합니다. 반환 텍스트는 스칼라 값이며 따옴표로 감싼 SQL 쿼리 조각이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
