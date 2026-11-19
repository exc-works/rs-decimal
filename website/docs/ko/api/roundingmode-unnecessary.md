---
locale: ko
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
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

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
