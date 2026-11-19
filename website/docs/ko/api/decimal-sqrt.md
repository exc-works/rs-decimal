---
locale: ko
source_revision: 7f9e2f6ed775898aae666efe4610ba891f54aadb6987e2a347de88c3c59500ea
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sqrt
---
# Decimal::sqrt

제곱근을 계산합니다. 차수 매개변수는 없습니다. 음수 입력은 NegativeRoot를 반환합니다. 기본 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 최근접 짝수 반올림을 사용합니다. 정밀도 0의 sqrt(6.25)처럼 정확히 중간인 근은 2로 반올림합니다. approx_root라는 이름이어도 Rust는 정수 비교로 반올림을 증명합니다. 코어 계산/반복 예산을 소진하면 NoConvergence를 반환합니다. 입력은 빌려 쓰며 바꾸지 않고 기본 0을 포함한 결과는 초기화됩니다.

```rust
fn sqrt (& self) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="roots-decimal-sqrt" locale="ko" />

## 관련 메서드

[sqrt](./decimal-sqrt) · [approx_root_with_prec](./decimal-approx-root-with-prec) · [power](./decimal-power)
