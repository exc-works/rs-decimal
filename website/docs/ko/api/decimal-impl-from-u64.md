---
locale: ko
source_revision: fc3ed611d72b0cb1f91a16643e1645524bf0ecd7713a4a9f4adaaf5279855e77
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl From < u64 >"
---
# Decimal::impl From < u64 >

Rust From으로 u64을 정확히 변환합니다.

```rust
impl From<u64> for Decimal
```

## 일반적인 사용법과 주의사항

u64을 소비하고 소수 자릿수가 0인 초기화된 Decimal을 반환합니다. 부동소수점을 거치거나 반올림하지 않으며 해당 원본 정수 형식의 변환은 실패하지 않습니다. 0도 Decimal::default()와 달리 초기화됩니다. 원본 범위는 Rust 정수 형식을 따릅니다. isize/usize의 폭은 네이티브 대상에 따라 다릅니다. 브라우저 코드는 명시적인 명명 생성자 또는 ConfigValue의 signed/unsigned 변형을 사용합니다. Rust From/Into 구문은 네이티브 전용입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-from-u64" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
