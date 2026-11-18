---
locale: ko
source_revision: b8a64fe28faf9f699b61b2f86e331fd9c5b8333882ce85a4476ea07f56d57e1d
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl FromStr::from_str
---
# Decimal::impl FromStr::from_str

Rust parse와 FromStr로 소수 텍스트를 파싱합니다.

```rust
fn from_str (text : & str) -> Result < Self , Self :: Err >
```

## 일반적인 사용법과 의미

Decimal 또는 DecimalError를 반환합니다. 문법과 0 정규화는 new_from_string과 같습니다. 브라우저는 해당 이름의 메서드를 호출하며 Rust 트레이트 문법은 네이티브 전용입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fromstr-from-str" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
