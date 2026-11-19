---
locale: ko
source_revision: 347ad719aa4b33bd925c7dfe63b5e557c57b390b4d8d0b57d7aff0d9ca62ac93
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

왼쪽 피연산자를 소비하여 정확하게 계산합니다.

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 입력 중 큰 소수 자릿수로 초기화된 Decimal을 반환합니다. 수신 객체는 이동되며 other는 계속 사용할 수 있습니다. 저장 공간을 재사용할 수 있지만 증가하면 할당이 발생할 수 있습니다. JavaScript는 같은 래퍼를 수신 객체와 other에 동시에 쓰는 것을 이동 전에 거부합니다. Rust에 들어간 뒤에는 리소스 오류가 발생해도 수신 객체를 소비합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
