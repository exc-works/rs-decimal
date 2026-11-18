---
locale: ko
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
translation_provenance: en-US-korean-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

지정한 소수 자릿수에서 가장 가까운 값으로 반올림하며 중간이면 짝수 쪽을 택합니다.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 매개변수

`prec: isize` — 소수점 아래 자릿수입니다. 브리지는 JavaScript 정숫값을 받습니다.

## 반환값

기본 영 값을 입력해도 초기화된 Decimal을 반환합니다. 입력을 빌리며 정밀도를 높이면 끝에 0을 붙입니다.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 다른 반올림 모드는 [`rescale`](./decimal-rescale) 호출로 지정합니다.

## 오류

음수 정밀도가 반환하는 오류는 `InvalidPrecision`. 브리지는 해당 오류를 던집니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
