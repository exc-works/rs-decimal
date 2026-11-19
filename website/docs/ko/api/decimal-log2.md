---
locale: ko
source_revision: a329356af92d18ac765420533258b5c608220d0cf27b5772304a0e1b5b8d5f9e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::log2
---
# Decimal::log2

밑이 2인 로그를 계산합니다. Rust는 Result<Decimal, DecimalError>를, bridge는 Decimal 또는 구조화된 예외를 반환합니다. 매개변수 없음. 기본 출력 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 입력은 양수여야 하며 0과 음수는 InvalidLog를 반환합니다. 밑의 정확한 정수 거듭제곱은 정확한 정수 로그를 가지며 출력 소수 자릿수로 유지합니다. 코어는 값을 포함하는 구간을 사용하며 반올림을 증명한 경우에만 결과를 수락합니다. mode 매개변수 없이 최근접 짝수 반올림을 사용합니다. 코어 예산 안에서 증명하지 못하면 검증되지 않은 근삿값 대신 NoConvergence를 반환합니다. 입력은 변하지 않고 성공한 결과는 초기화됩니다.

```rust
fn log2 (& self) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2" locale="ko" />

## 관련 메서드

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
