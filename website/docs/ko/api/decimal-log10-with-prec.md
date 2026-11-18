---
locale: ko
source_revision: c8e9892bcbbaa01844a1c6c7f625a7680f4f756c37b722e825b006201909c45f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::log10_with_prec
---
# Decimal::log10_with_prec

밑이 10인 로그를 계산합니다. Rust는 Result<Decimal, DecimalError>를, bridge는 Decimal 또는 구조화된 예외를 반환합니다. prec: 음수가 아닌 isize를 정수 number로 나타내며 소수 자릿수를 선택합니다. 음수 정밀도는 정의역 검사 전에 InvalidPrecision을 반환합니다. 입력은 양수여야 하며 0과 음수는 InvalidLog를 반환합니다. 밑의 정확한 정수 거듭제곱은 정확한 정수 로그를 가지며 출력 소수 자릿수로 유지합니다. 코어는 값을 포함하는 구간을 사용하며 반올림을 증명한 경우에만 결과를 수락합니다. mode 매개변수 없이 최근접 짝수 반올림을 사용합니다. 코어 예산 안에서 증명하지 못하면 검증되지 않은 근삿값 대신 NoConvergence를 반환합니다. 입력은 변하지 않고 성공한 결과는 초기화됩니다.

```rust
fn log10_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10-with-prec" locale="ko" />

## 브라우저 제한

이 그룹에는 추가 [리소스 제한](../resource-policy)이 있습니다. 출력 정밀도는 최대 128, 입력 소수 자릿수는 최대 256, 입력 계수는 최대 4096비트입니다. 일반 exp 계산은 입력 절댓값도 최대 64여야 합니다. 코어가 직접 0으로 반올림된다고 증명하는 음수 입력은 빠른 경로를 유지합니다. 입력 자릿수가 크면 인수 없는 메서드의 기본 정밀도가 128을 넘을 수 있으므로 with_prec로 허용된 출력 정밀도를 명시하세요. 리소스 오류와 코어 NoConvergence는 다릅니다. Stop은 실행을 중단하며 다음 실행은 새 Wasm 인스턴스를 만듭니다.

## 관련 메서드

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
