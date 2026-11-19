---
locale: ko
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
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

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
