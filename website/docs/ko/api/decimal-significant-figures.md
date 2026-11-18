---
locale: ko
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
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

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
