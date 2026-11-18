---
locale: ko
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
translation_provenance: en-US-korean-reviewed
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

소유한 계수와 소수 자릿수로 초기화된 값을 생성합니다.

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

coefficient: BigInt, scale: usize(음수가 아닌 소수 자릿수). 후행 0과 값이 0일 때의 소수 자릿수도 유지하며 텍스트 파서 정규화를 사용하지 않습니다. isize::MAX를 넘으면 Rust는 Overflow를 반환합니다. 브라우저는 정확한 bigint와 wasm32 usize 범위의 정수 number를 사용합니다. i32::MAX를 넘는 소수 자릿수는 Overflow를 유지하며, 그 외에 1000을 넘는 값은 playground 리소스 제한을 초과합니다. JS bigint는 불변이므로 이동되는 래퍼가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
