---
locale: ko
source_revision: dfbad807bf85fc60fbbfb720800bbf36eb483a86897e4245a66f95ff012fb636
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

왼쪽 피연산자를 소비하여 정확하게 계산합니다.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 입력 중 큰 소수 자릿수로 초기화된 Decimal을 반환합니다. 수신 객체는 이동되며 other는 계속 사용할 수 있습니다. 저장 공간을 재사용할 수 있지만 증가하면 할당이 발생할 수 있습니다. bridge는 같은 래퍼를 수신 객체와 other에 동시에 쓰는 것을 이동 전에 거부합니다. Rust에 들어간 뒤에는 리소스 오류가 발생해도 수신 객체를 소비합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
