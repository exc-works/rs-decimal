---
locale: ko
source_revision: 6c8f91b0409cc71714a4b9926acf34a3cf0aa635f8904f2e607ed3243fb8b30a
translation_provenance: en-US-korean-reviewed
api_id: Decimal::power
---
# Decimal::power

원래 소수 자릿수로 정수 거듭제곱을 계산합니다.

```rust
fn power (& self , power : i64) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

power: i64를 bigint로 전달합니다. 입력의 소수 자릿수에 맞춰 최근접 짝수 반올림을 하며 음수 지수는 역수를 구합니다. 2.00의 -3승은 0.12이지만 정수 자릿수의 2의 -3승은 0입니다. 0의 0승은 초기화된 1이고 0의 음수 승은 DivideByZero입니다. 코어 중간값 한계를 넘는 큰 지수는 할당 전에 Overflow를 반환합니다. 브라우저의 중간 소수 자릿수/계수 한계는 더 작으며 0과 +/-1은 코어의 빠른 경로를 유지합니다. 입력은 빌려 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-power" locale="ko" />

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
