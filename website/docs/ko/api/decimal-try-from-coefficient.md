---
locale: ko
source_revision: 3430467a0f3b43857ec837a38512411503e4016266c068e3065f8bd60687547d
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

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
