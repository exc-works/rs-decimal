---
locale: ko
source_revision: e6982e644ce0db12869f83bce30b6a469943f72b611efb52c20fbf5be010ea23
translation_provenance: en-US-korean-reviewed
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

양의 무한대 방향으로 올립니다.

```rust
Ceiling = 2
```

## 매개변수, 반환값, 일반적인 사용법

정수 판별값은 2입니다. 매직 넘버 대신 이름이 있는 값을 사용하세요. 규칙은 양수와 음수 모두에 적용됩니다. Down과 Up은 음/양의 무한대가 아니라 0을 기준으로 한 방향입니다. Half 방식은 정확한 중간값에서만 다릅니다. 별도의 Floor 열거값은 없으므로 해당 방향에는 floor_with_prec를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="ko" />

## 브라우저 제한

정수 매개변수는 wasm32 범위 안에 있어야 합니다. 브라우저는 목표 소수 자릿수, 계수 증가, 호출 횟수를 포함하는 [리소스 정책](../resource-policy)도 적용합니다. 리소스 오류와 수치 라이브러리 오류는 구분합니다. 빌림 메서드는 입력을 소비하지 않으며 Rust 전용 트레이트 문법은 네이티브에서 검사한다고 명시합니다.

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
