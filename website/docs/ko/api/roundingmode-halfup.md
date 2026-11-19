---
locale: ko
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

가장 가까운 값으로 반올림하며 중간이면 0에서 멀어지는 쪽을 택합니다.

```rust
HalfUp = 3
```

## 매개변수, 반환값, 일반적인 사용법

정수 판별값은 3입니다. 매직 넘버 대신 이름이 있는 값을 사용하세요. 규칙은 양수와 음수 모두에 적용됩니다. Down과 Up은 음/양의 무한대가 아니라 0을 기준으로 한 방향입니다. Half 방식은 정확한 중간값에서만 다릅니다. 별도의 Floor 열거값은 없으므로 해당 방향에는 floor_with_prec를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
