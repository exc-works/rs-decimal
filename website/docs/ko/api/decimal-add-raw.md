---
locale: ko
source_revision: 872ef2a43e7ac5d06f4ea120672471b0a862ee6f8a70e48db9f1f9367f2518f7
translation_provenance: en-US-korean-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

스케일 적용 전 계수를 i64 정수만큼 바꿉니다.

```rust
fn add_raw (& self , value : i64) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

value: i64를 bridge에서는 bigint로 나타냅니다. 소수 자릿수를 유지하므로 자릿수 2에서 1을 더하거나 빼면 수치는 1이 아니라 0.01만큼 변합니다. Rust는 새 Decimal을 반환합니다. bridge는 number와 i64 범위 밖의 bigint를 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
