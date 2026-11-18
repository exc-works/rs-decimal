---
locale: ko
source_revision: d358fea1722186e0df2e76e232b3793bff18d2e3a10cb6ef0ae0a724a40da40b
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

반올림 방식에 Rust PartialEq 구현을 사용합니다.

```rust
impl PartialEq for RoundingMode
```

## 매개변수, 반환값, 일반적인 사용법

RoundingMode는 작은 Copy 열거형입니다. 복사해도 선택한 규칙을 유지하고, 동등성은 변형을 비교하며 Debug는 이름을 출력합니다. 이 예제는 네이티브 Rust 트레이트 문법을 검사합니다. 브라우저 모드는 불변 숫자 값입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
