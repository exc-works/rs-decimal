---
locale: ko
source_revision: fa1a3656d76ca2759cc49aed5b71589136f0ed7e84a57f811be7062ea9aa8860
translation_provenance: en-US-korean-reviewed
api_id: Decimal::into_coefficient
---
# Decimal::into_coefficient

스케일 적용 전 정수 계수를 이동하여 꺼냅니다.

```rust
fn into_coefficient (self) -> BigInt
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust에서는 BigInt, JS에서는 bigint를 반환합니다. 소수 자릿수와 초기화 상태는 사라지므로 1.2300은 12300이 됩니다. 기본 0은 0입니다. Decimal은 소비되어 다시 읽을 수 없습니다. bridge에서 소비하지 않고 스냅샷을 얻으려면 coefficient를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-coefficient" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
