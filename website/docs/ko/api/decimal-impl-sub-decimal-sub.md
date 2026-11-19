---
locale: ko
source_revision: c4a0a7eefa609b0c57e86546936b34ea64a5ece51d5f57a28f19151facde5f69
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Sub < & Decimal >::sub"
---
# Decimal::impl Sub < & Decimal >::sub

빌리는 Rust Sub 구문을 사용합니다.

```rust
fn sub (self , other : & Decimal) -> Decimal
```

## 일반적인 사용법과 주의사항

&Decimal에 구현되어 피연산자를 계속 사용할 수 있습니다. 정확한 결과는 피연산자 중 큰 소수 자릿수를 사용하며 초기화됩니다. 연관 Output 형식은 Decimal입니다. 음수 결과를 허용하며 safe_add/safe_sub의 비음수 조건은 적용하지 않습니다. 트레이트는 같은 이름의 코어 메서드에 위임합니다. 브라우저 JS 연산자는 오버로드되지 않으므로 JavaScript의 add/sub/neg를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-sub" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
