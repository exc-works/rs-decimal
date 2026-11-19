---
locale: ko
source_revision: 8de700fe2bea77817d4cc64ce868f77fd5301e630577d698cc1cf5a3b4f76ce5
translation_provenance: en-US-korean-reviewed
api_id: "reexport::PRECISION_FIXED_SIZE"
---
# reexport::PRECISION_FIXED_SIZE

바이너리 정밀도 접두부의 바이트 길이를 읽습니다.

```rust
pub use binary::PRECISION_FIXED_SIZE as PRECISION_FIXED_SIZE
```

## 매개변수, 반환값, 일반적인 사용법

값이 4인 Rust usize 상수로, Go 호환 바이너리 전송 형식의 빅엔디언 소수 자릿수 접두부 바이트 길이입니다. 십진 숫자 개수나 정밀도 한계가 아닙니다. 브라우저는 Rust에서 가져온 읽기 전용 number를 공개합니다. 크레이트 루트 재내보내기는 Rust 가져오기 문법입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-reexport-precision-fixed-size" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
