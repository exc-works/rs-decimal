---
locale: ko
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
translation_provenance: en-US-korean-reviewed
api_id: Decimal::round
---
# Decimal::round

가장 가까운 정수로 반올림하며 중간이면 짝수를 택합니다.

```rust
fn round (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 소수 자릿수 0인 새 Decimal을 반환합니다. 여기서는 최근접 짝수 반올림이 기본이며 JavaScript Math.round의 중간값 처리와 다릅니다. 입력은 빌려 사용하며 변경하지 않습니다. 기본 0이 입력이어도 결과는 초기화됩니다. 정밀도를 높이면 수치를 바꾸지 않고 0을 추가합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
