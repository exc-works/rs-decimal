---
locale: ko
source_revision: 3fdc16376a6b5df85a2ce7cc24f6688fcdf41771e793df75d13c0d1b90a448b1
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

스케일 적용 전 계수를 i64 정수만큼 바꿉니다.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

value: i64를 JavaScript에서는 bigint로 나타냅니다. 소수 자릿수를 유지하므로 자릿수 2에서 1을 더하거나 빼면 수치는 1이 아니라 0.01만큼 변합니다. Rust는 새 Decimal을 반환합니다. JavaScript는 number와 i64 범위 밖의 bigint를 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
