---
locale: ko
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

정확한 결과만 허용합니다.

```rust
Unnecessary = 6
```

## 매개변수, 반환값, 일반적인 사용법

정수 판별값은 6입니다. 매직 넘버 대신 이름이 있는 값을 사용하세요. 규칙은 양수와 음수 모두에 적용됩니다. 0인 숫자를 버리는 것은 정확하지만 0이 아닌 숫자를 하나라도 버리면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
