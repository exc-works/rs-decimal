---
locale: ko
source_revision: 50b84c140827ca9ee14623ef754490d47ed0de164e4d5e55011356b86e00545d
translation_provenance: en-US-korean-reviewed
api_id: Decimal::shift
---
# Decimal::shift

소수에 정확한 10의 거듭제곱을 곱합니다.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

places: isize/정수 number. 양수이면 소수점을 오른쪽, 음수이면 왼쪽으로 옮깁니다. rescale과 달리 수치를 바꿉니다. 0이 아닌 값의 이동으로 소수 자릿수가 i32::MAX를 넘으면 코어는 Overflow를 반환합니다. shift(0)도 기본 0을 초기화합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
