---
locale: ko
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
translation_provenance: en-US-korean-reviewed
api_id: reexport::ten
---
# reexport::ten

rs_decimal 크레이트 루트에서 ten를 가져옵니다.

```rust
pub use decimal::ten as ten
```

## 일반적인 사용법과 의미

rs_decimal::ten를 사용하세요. 같은 항목의 공개 가져오기 경로이며 추가 수치 구현이 아닙니다. JavaScript에는 Rust use 문법이 없으므로 대응하는 bridge 참조 문서를 확인하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
