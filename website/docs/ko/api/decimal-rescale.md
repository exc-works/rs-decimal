---
locale: ko
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
translation_provenance: en-US-korean-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

명시적인 반올림 방식으로 소수 자릿수를 바꿉니다.

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec는 유효 숫자가 아니라 소수 자릿수이며 mode는 일곱 RoundingMode 값 중 하나입니다. 정밀도를 높이면 0을 추가합니다. 낮출 때는 버리는 0이 아닌 숫자만 반올림에 영향을 줍니다. 음수 정밀도는 InvalidPrecision이며 Unnecessary에서 반올림이 필요하면 RoundUnnecessary를 반환합니다. 결과는 기본 0도 초기화하며 빌린 입력은 변하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
