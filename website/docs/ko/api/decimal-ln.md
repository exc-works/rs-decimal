---
locale: ko
source_revision: d7c263dcbf87f527f8688d8c67f916ab9df964fed5eeb802c12da2b176a5e991
translation_provenance: en-US-korean-reviewed
api_id: Decimal::ln
---
# Decimal::ln

자연로그를 계산합니다. Rust는 Result<Decimal, DecimalError>를, bridge는 Decimal 또는 구조화된 예외를 반환합니다. 매개변수 없음. 기본 출력 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 입력은 양수여야 하며 0과 음수는 InvalidLog를 반환합니다. ln(1)은 정확히 0입니다. 코어는 값을 포함하는 구간을 사용하며 반올림을 증명한 경우에만 결과를 수락합니다. mode 매개변수 없이 최근접 짝수 반올림을 사용합니다. 코어 예산 안에서 증명하지 못하면 검증되지 않은 근삿값 대신 NoConvergence를 반환합니다. 입력은 변하지 않고 성공한 결과는 초기화됩니다.

```rust
fn ln (& self) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln" locale="ko" />

## 관련 메서드

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
