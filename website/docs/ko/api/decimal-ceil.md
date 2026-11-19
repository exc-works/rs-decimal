---
locale: ko
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
translation_provenance: en-US-korean-reviewed
api_id: Decimal::ceil
---
# Decimal::ceil

양의 무한대 방향으로 올려 정수로 만듭니다.

```rust
fn ceil (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 소수 자릿수 0인 새 Decimal을 반환합니다. 입력은 빌려 사용하며 변경하지 않습니다. 기본 0이 입력이어도 결과는 초기화됩니다. 정밀도를 높이면 수치를 바꾸지 않고 0을 추가합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
