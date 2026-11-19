---
locale: ko
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
translation_provenance: en-US-korean-reviewed
api_id: Decimal::min
---
# Decimal::min

두 수치 중 작은 값을 복사합니다.

```rust
fn min (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 독립적인 Decimal을 반환합니다. 수치가 같으면 첫 피연산자의 소수 자릿수와 초기화 상태를 유지합니다. Rust에서는 Decimal::min(&left, &right)를 사용하여 값을 소비하는 Ord::min 트레이트 메서드가 선택되지 않도록 하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
