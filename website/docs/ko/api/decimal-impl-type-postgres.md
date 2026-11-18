---
locale: ko
source_revision: 7dc5d9b6c0f5030d430dc8fa4c821516f6b5bfe8979234b7de2b0cf0b1d59588
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Type < Postgres >"
---
# Decimal::impl Type < Postgres >

네이티브 SQLx PostgreSQL NUMERIC 연동을 사용합니다.

```rust
impl Type < Postgres > for Decimal
```

## 일반적인 사용법과 주의사항

postgres 기능과 SQLx 0.8의 PostgreSQL 형식이 필요합니다. 이 구현은 네이티브 전용이며 브라우저 Playground는 데이터베이스에 연결하지 않습니다. type_info()는 NUMERIC OID 1700을 반환합니다. compatible(ty)는 이 OID만 허용하며 float8, 정수, 임의의 텍스트 형식은 허용하지 않습니다. 암묵적인 부동소수점 변환을 보장하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-type-postgres" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
