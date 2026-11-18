---
locale: ko
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

정수 반올림 방식 코드를 검사합니다.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

value: i32/정수 number. 0–6에는 해당 방식을 반환하고 다른 i32 값에는 InvalidArgument를 반환합니다. 브라우저의 소수, 비유한 값, i32 범위 밖의 값은 Rust 호출 전 매개변수 검사에서 실패합니다. bridge는 Rust 검증 함수를 호출하여 숫자 형태의 방식을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
