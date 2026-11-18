---
locale: ko
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
translation_provenance: en-US-korean-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

수치를 바꾸지 않고 불필요한 소수 부분의 후행 0을 제거합니다.

```rust
fn strip_trailing_zeros (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 독립적인 Decimal을 반환합니다. 소수 자릿수가 양수일 때만 10의 인수를 제거하므로 1200 같은 정수의 0은 버리지 않습니다. 모든 수치 0은 소수 자릿수 0인 초기화된 0이 됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
