---
locale: ko
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
translation_provenance: en-US-korean-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

지정한 최대 유효 숫자 개수로 반올림합니다.

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

figures: 양의 isize/정수 number, mode: 반올림 방식. 소수 자릿수와 달리 유효 숫자는 계수의 숫자를 셉니다. 정밀도를 추가하지 않으며 정수 부분을 반올림할 수 있습니다(유효 숫자 3개에서 1234.5 → 1230). 양수가 아닌 figures는 InvalidArgument, Unnecessary에서 반올림이 필요하면 RoundUnnecessary를 반환합니다. 계수의 자리 올림 가능성을 브라우저 예산에 먼저 대조합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
