---
locale: ko
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

반올림 방식에 Rust Debug 구현을 사용합니다.

```rust
impl Debug for RoundingMode
```

## 매개변수, 반환값, 일반적인 사용법

RoundingMode는 작은 Copy 열거형입니다. 복사해도 선택한 규칙을 유지하고, 동등성은 변형을 비교하며 Debug는 이름을 출력합니다. 이 예제는 네이티브 Rust 트레이트 문법을 검사합니다. 브라우저 모드는 불변 숫자 값입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
