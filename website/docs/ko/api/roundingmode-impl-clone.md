---
locale: ko
source_revision: be26937ee4bcd1a30b10deb30b4ced946f0371e3015f1e537f8c85709084d801
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

반올림 방식에 Rust Clone 구현을 사용합니다.

```rust
impl Clone for RoundingMode
```

## 매개변수, 반환값, 일반적인 사용법

RoundingMode는 작은 Copy 열거형입니다. 복사해도 선택한 규칙을 유지하고, 동등성은 변형을 비교하며 Debug는 이름을 출력합니다. 이 예제는 네이티브 Rust 트레이트 문법을 검사합니다. 브라우저 모드는 불변 숫자 값입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
