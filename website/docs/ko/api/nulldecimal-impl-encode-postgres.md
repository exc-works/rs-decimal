---
locale: ko
source_revision: f3551e314b31f667d7ca138dd63f4da03440121f2536e11a0e604ede2dbe9e2b
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::impl Encode < '_ , Postgres >"
---
# NullDecimal::impl Encode < '_ , Postgres >

네이티브 SQLx PostgreSQL NUMERIC 연동을 사용합니다.

```rust
impl Encode < '_ , Postgres > for NullDecimal
```

## 일반적인 사용법과 주의사항

postgres 기능과 SQLx 0.8의 PostgreSQL 형식이 필요합니다. 이 구현은 네이티브 전용이며 브라우저 Playground는 데이터베이스에 연결하지 않습니다. encode_by_ref(&self, &mut PgArgumentBuffer)는 `Result<IsNull, BoxDynError>`를 반환합니다. 빌리기는 값을 유지하며 인코딩에 성공한 뒤에만 기존 인수 버퍼에 바이트를 추가합니다. 유효하지 않은 nullable은 바이트 없이 IsNull::Yes를 반환합니다. 유효한 값은 내부 미초기화 0을 포함한 Decimal을 인코딩합니다. 바이너리 형식은 빅 엔디언 10000진 그룹을 사용하고 표시 자릿수를 보존하며 float/BigDecimal을 거치지 않습니다. 소수 자릿수 16383 초과 또는 정수 자릿수 131072 초과는 거부합니다. 인코딩 오류는 대상 버퍼를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-encode-postgres" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 기본 네이티브 문서 테스트 실행기가 이 예제를 실행합니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
