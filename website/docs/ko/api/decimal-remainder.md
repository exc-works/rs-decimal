---
locale: ko
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
translation_provenance: en-US-korean-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

정수 부분과 스케일 적용 전 소수 부분의 나머지를 나눕니다.

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. (BigInt, BigInt)를 반환하며 [bigint, bigint]에 대응합니다. 첫 값은 0 방향으로 버린 정수이고 두 번째는 부호 있는 계수 나머지이며 Decimal 소수 부분이 아닙니다. -12.750의 결과는 (-12, -750)입니다. 두 번째 값을 -0.750으로 해석하려면 원래 소수 자릿수 3이 필요합니다. 기본 0은 (0, 0)을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
