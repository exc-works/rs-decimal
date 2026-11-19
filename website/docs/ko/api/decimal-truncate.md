---
locale: ko
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::truncate
---
# Decimal::truncate

0 방향으로 버려 정수로 만듭니다.

```rust
fn truncate (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 소수 자릿수 0인 새 Decimal을 반환합니다. 음수에서는 0 방향 버림과 floor가 다릅니다. -1.5를 버리면 -1이지만 floor는 -2입니다. 입력은 빌려 사용하며 변경하지 않습니다. 기본 0이 입력이어도 결과는 초기화됩니다. 정밀도를 높이면 수치를 바꾸지 않고 0을 추가합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
