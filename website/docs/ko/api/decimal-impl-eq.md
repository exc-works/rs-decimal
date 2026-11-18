---
locale: ko
source_revision: cfbab45ed5115c58df815ab40afe368434f97862eeaecc53b795f1f752fe2e25
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl Eq
---
# Decimal::impl Eq

Rust에서 완전한 동등성가 필요한 곳에 Decimal을 사용합니다.

```rust
impl Eq for Decimal
```

## 일반적인 사용법과 의미

Eq는 메서드가 없는 마커 트레이트입니다. 동등성은 수치를 비교하며 소수 자릿수와 초기화 상태를 무시합니다. JavaScript 객체 동일성을 뜻하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-eq" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
