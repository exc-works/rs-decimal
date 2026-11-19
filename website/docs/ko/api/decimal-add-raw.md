---
locale: ko
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
translation_provenance: en-US-korean-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

스케일 적용 전 계수를 i64 정수만큼 바꿉니다.

```rust
fn add_raw (& self , value : i64) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

value: i64를 JavaScript에서는 bigint로 나타냅니다. 소수 자릿수를 유지하므로 자릿수 2에서 1을 더하거나 빼면 수치는 1이 아니라 0.01만큼 변합니다. Rust는 새 Decimal을 반환합니다. JavaScript는 number와 i64 범위 밖의 bigint를 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
