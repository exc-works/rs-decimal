---
locale: ko
source_revision: cd65ee6368cc890889ec3fceb39f3b12814d9f2d0fe811e6c57c730bc5d81b5b
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::value"
---
# NullDecimal::value

정규 SQL 값을 얻습니다.

```rust
fn value (& self) -> Option < String >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Option<String>`/string | null을 반환합니다. 무효인 nullable은 숨겨진 내부 데이터와 무관하게 None/null입니다. 유효한 값은 정규 소수 텍스트를 반환하며 내부가 미초기화 0이면 "0"입니다. 소수 자릿수나 초기화를 바꾸지 않고 소수 부분의 후행 0을 제거합니다. 반환 텍스트는 스칼라 값이며 따옴표로 감싼 SQL 쿼리 조각이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-nulldecimal-value" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
