---
locale: ko
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Rust Default로 값이 없는 nullable을 만듭니다.

```rust
impl Default for NullDecimal
```

## 일반적인 사용법과 의미

유효성과 내부 초기화 상태가 모두 false입니다. JavaScript는 NullDecimal.default()를 제공합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
