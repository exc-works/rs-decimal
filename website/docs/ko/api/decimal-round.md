---
locale: ko
source_revision: dfbccd4ef90aecedf2d24565d11016e8068251bd103af1f82ea5d3b25f25cee0
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

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
