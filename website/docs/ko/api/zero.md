---
locale: ko
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
translation_provenance: en-US-korean-reviewed
api_id: zero
---
# zero

소수 자릿수 0인 초기화된 0을 생성합니다.

```rust
fn zero () -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

매개변수 없음. 새 Decimal을 반환합니다. Decimal::default()와 수치로는 같지만 결과는 초기화되어 있습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="ko" />

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
