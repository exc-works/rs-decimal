---
locale: ko
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
translation_provenance: en-US-korean-reviewed
api_id: min
---
# min

자유 함수로 두 소수 중 작은 값을 복사합니다.

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

a와 b를 &Decimal로 빌려 독립적인 Decimal을 반환합니다. 수치가 같으면 a의 소수 자릿수와 초기화 상태를 유지합니다. Decimal::min과 동일한 라이브러리 동작을 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="ko" />

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
