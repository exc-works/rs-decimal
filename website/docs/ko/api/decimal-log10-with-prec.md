---
locale: ko
source_revision: fc8796bf45611497be2998723d098452a9849a3946a05ba9bf67b1cf99b43481
translation_provenance: en-US-korean-reviewed
api_id: Decimal::log10_with_prec
---
# Decimal::log10_with_prec

밑이 10인 로그를 계산합니다. prec: 음수가 아닌 isize를 정수 number로 나타내며 소수 자릿수를 선택합니다. 음수 정밀도는 정의역 검사 전에 InvalidPrecision을 반환합니다. 입력은 양수여야 하며 0과 음수는 InvalidLog를 반환합니다. 밑의 정확한 정수 거듭제곱은 정확한 정수 로그를 가지며 출력 소수 자릿수로 유지합니다. 코어는 값을 포함하는 구간을 사용하며 반올림을 증명한 경우에만 결과를 수락합니다. mode 매개변수 없이 최근접 짝수 반올림을 사용합니다. 코어 예산 안에서 증명하지 못하면 검증되지 않은 근삿값 대신 NoConvergence를 반환합니다. 입력은 변하지 않고 성공한 결과는 초기화됩니다.

```rust
fn log10_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10-with-prec" locale="ko" />

## 관련 메서드

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
