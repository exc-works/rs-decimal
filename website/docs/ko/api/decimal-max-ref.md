---
locale: ko
source_revision: 78bc5479760ac53643504ad7a87ce54e9faf3966cce17b807b0ad11092850a19
translation_provenance: en-US-korean-reviewed
api_id: Decimal::max_ref
---
# Decimal::max_ref

수치 비교로 원래 피연산자를 선택합니다.

```rust
fn max_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 입력 수명에 맞는 공유 참조를 반환합니다. 값이 같으면 self를 선택하여 소수 자릿수와 초기화 상태를 유지합니다. bridge는 복사본이 아니라 해당 원래 래퍼를 반환합니다. 래퍼를 이동하거나 해제하면 모든 JS 별칭도 무효가 됩니다. JS는 Rust 빌림 수명을 강제할 수 없습니다. 독립적으로 소유하는 결과에는 max/min을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-max-ref" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
