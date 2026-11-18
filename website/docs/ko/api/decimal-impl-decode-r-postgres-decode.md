---
locale: ko
source_revision: 21da10a6e76a84c5146b6a083d73a886edce7505fb47206e66d109e57f693aeb
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Decode < 'r , Postgres >::decode"
---
# Decimal::impl Decode < 'r , Postgres >::decode

네이티브 SQLx PostgreSQL NUMERIC 연동을 사용합니다.

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## 일반적인 사용법과 주의사항

postgres 기능과 SQLx 0.8의 PostgreSQL 형식이 필요합니다. 이 구현은 네이티브 전용이며 브라우저 Playground는 데이터베이스에 연결하지 않습니다. decode(PgValueRef)는 `Result<Self, BoxDynError>`를 반환합니다. SQLx가 빌린 데이터베이스 값을 제공하므로 비공개 전송 표현을 직접 만들지 마세요. 바이너리와 텍스트 NUMERIC은 소수 자릿수가 있는 0을 포함해 값과 표시 자릿수를 보존합니다. SQL NULL은 거부합니다. nullable 열에는 NullDecimal 또는 `Option<Decimal>`을 사용하세요. null이 아닌 결과는 초기화됩니다. 잘못된 NUMERIC과 유한하지 않은 NUMERIC은 반올림 없이 거부합니다. 이 예제는 CI에서 전용 PostgreSQL 서버로 실행합니다. 브라우저에는 외부 데이터베이스가 필요하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres-decode" locale="ko" />

## 실행

이 항목은 네이티브 Rust 트레이트 구문이며 같은 이름의 JS 연산자나 SQLx 객체를 제공하지 않습니다. 전용 PostgreSQL 서버와 DATABASE_URL을 준비한 뒤 `cargo run --locked -p decimal-web-examples --features postgres-live`로 예제를 실행하세요. 이 프로필이 없으면 명시적으로 건너뛰었다고 보고하며 데이터베이스 검증 성공으로 세지 않습니다.

## 관련 메서드

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
