---
locale: ko
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
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

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
