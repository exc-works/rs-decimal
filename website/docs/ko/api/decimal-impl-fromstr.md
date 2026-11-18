---
locale: ko
source_revision: 65703feb0e11af9dfe348c058c514dbf10ab13c1d2a720632926e57fee628fa9
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl FromStr
---
# Decimal::impl FromStr

Rust parse와 FromStr로 소수 텍스트를 파싱합니다.

```rust
impl FromStr for Decimal
```

## 일반적인 사용법과 의미

Decimal 또는 DecimalError를 반환합니다. 문법과 0 정규화는 new_from_string과 같습니다. 브라우저는 해당 이름의 메서드를 호출하며 Rust 트레이트 문법은 네이티브 전용입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
