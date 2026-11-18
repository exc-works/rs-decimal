---
locale: ko
source_revision: 02d9f7c1451db386b5c6959aa8f2c1d403c00069d00df185eafd17eb7f7b188e
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl PgHasArrayType"
---
# Decimal::impl PgHasArrayType

네이티브 SQLx PostgreSQL NUMERIC 연동을 사용합니다.

```rust
impl PgHasArrayType for Decimal
```

## 일반적인 사용법과 주의사항

postgres 기능과 SQLx 0.8의 PostgreSQL 형식이 필요합니다. 이 구현은 네이티브 전용이며 브라우저 Playground는 데이터베이스에 연결하지 않습니다. array_type_info()는 NUMERIC[] OID 1231을 반환하여 SQLx 배열 바인딩을 지원합니다. NullDecimal 배열 요소는 유효성으로 SQL NULL을 보존합니다. 전체 배열의 인코딩/디코딩에는 네이티브 SQLx 쿼리를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-pghasarraytype" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
