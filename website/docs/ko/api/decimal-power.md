---
locale: ko
source_revision: 01c476851989c18cecc0970b361010eee7cb4cfd902d7e4240be18102abc0899
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

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
