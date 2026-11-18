---
locale: ko
source_revision: 45b2b4aa308ba6b40046aead6dddbc34fd9a06d38be18fb996c0bc76cff67656
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl FromStr::Err
---
# Decimal::impl FromStr::Err

Decimal의 FromStr 구현이 반환하는 오류 타입을 나타냅니다.

```rust
type Err = DecimalError
```

## 일반적인 사용법과 의미

연관 타입 `<Decimal as FromStr>::Err`는 `DecimalError`입니다. 잘못된 텍스트는 패닉 대신 이 오류를 반환합니다. 오류 종류로 잘못된 문법과 다른 실패를 구분하세요. Rust 타입 별칭이며 JavaScript 생성자가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-err" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
