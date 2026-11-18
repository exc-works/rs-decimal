---
locale: ko
source_revision: e228552c41e4946b118a075a0bdaa811f6ff511347af55baa0908ccb2c361609
translation_provenance: en-US-korean-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

0 방향으로 버려 소수 자릿수를 바꿉니다.

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize/정수 number. Decimal을 반환하며 음수 정밀도는 InvalidPrecision입니다. Down은 음의 무한대가 아니라 0 방향입니다. 정밀도를 높이면 0을 추가합니다. 입력은 변하지 않고 결과는 초기화됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
