---
locale: ko
source_revision: f662d4a18feb3987ba22a3e426a493d661feab0dd71696533c5ec29aefece86d
translation_provenance: en-US-korean-reviewed
api_id: Decimal::exp
---
# Decimal::exp

지수 함수 e^x를 계산합니다. 매개변수 없음. 기본 출력 정밀도는 max(입력 소수 자릿수, 20)이며 유효 숫자 20개가 아닙니다. 코어에서는 모든 실수 입력을 허용합니다. exp(0)은 초기화된 1이며 충분히 음수인 인수는 요청 정밀도에서 초기화된 0으로 정확히 반올림될 수 있습니다. 코어는 값을 포함하는 구간을 사용하며 반올림을 증명한 경우에만 결과를 수락합니다. mode 매개변수 없이 최근접 짝수 반올림을 사용합니다. 코어 예산 안에서 증명하지 못하면 검증되지 않은 근삿값 대신 NoConvergence를 반환합니다. 입력은 변하지 않고 성공한 결과는 초기화됩니다.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## 일반적인 사용법

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="ko" />

## 관련 메서드

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
