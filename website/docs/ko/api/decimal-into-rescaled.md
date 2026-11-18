---
locale: ko
source_revision: f56b3cc9eb8667e7cf067d7d1256d88fafb84d3cb5b89748b59c7df428bab4cf
translation_provenance: en-US-korean-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

소수를 소비하면서 소수 자릿수를 바꿉니다.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize, mode: RoundingMode. rescale처럼 반올림하거나 0을 추가하며 결과는 항상 초기화됩니다. Rust가 InvalidPrecision, RoundUnnecessary 또는 브라우저 리소스 오류를 반환해도 수신 객체는 이동됩니다. 잘못된 JS 타입/범위는 Rust에 들어가기 전에 거부하므로 소비하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
