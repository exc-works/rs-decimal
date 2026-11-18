---
locale: ko
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode
---
# RoundingMode

일곱 가지 명시적인 반올림 규칙 중 하나를 선택합니다.

```rust
pub enum RoundingMode
```

## 매개변수, 반환값, 일반적인 사용법

Rust에서는 이름이 있는 열거 변형, bridge에서는 RoundingMode 필드를 사용하세요. Rust 판별값 0–6은 브라우저 숫자와 일치합니다. Down은 0 방향, Up은 0에서 멀어지는 방향, Ceiling은 양의 무한대 방향입니다. HalfUp/HalfDown/HalfEven은 정확한 중간값에서 각각 0에서 멀어지는 쪽/0 방향/짝수 쪽을 고릅니다. Unnecessary는 정확성을 요구합니다. 암묵적인 전역 반올림 설정은 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
