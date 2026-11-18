---
locale: ko
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
translation_provenance: en-US-korean-reviewed
api_id: Decimal::shift
---
# Decimal::shift

소수에 정확한 10의 거듭제곱을 곱합니다.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

places: isize/정수 number. 양수이면 소수점을 오른쪽, 음수이면 왼쪽으로 옮깁니다. rescale과 달리 수치를 바꿉니다. 0이 아닌 값의 이동으로 소수 자릿수가 i32::MAX를 넘으면 코어는 Overflow를 반환합니다. 브라우저의 증가/자릿수 검사는 할당 전에 별도 리소스 오류를 반환할 수도 있습니다. shift(0)도 기본 0을 초기화합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
