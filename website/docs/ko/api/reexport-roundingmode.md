---
locale: ko
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
translation_provenance: en-US-korean-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

크레이트 루트에서 반올림 방식을 가져옵니다.

```rust
pub use rounding::RoundingMode as RoundingMode
```

## 매개변수, 반환값, 일반적인 사용법

rs_decimal::RoundingMode를 사용하세요. 같은 열거형이며 두 번째 구현이 아닙니다. 브라우저는 Rust 가져오기 문법 없이 이름이 있는 RoundingMode 객체를 공개합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
