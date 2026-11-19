---
locale: ko
source_revision: 65435d6f54d8777702cf2f629258b85e312b71241d8a0725f7c209d6e7b4a200
translation_provenance: en-US-korean-reviewed
api_id: "PRECISION_FIXED_SIZE"
---
# PRECISION_FIXED_SIZE

바이너리 정밀도 접두부의 바이트 길이를 읽습니다.

```rust
const PRECISION_FIXED_SIZE: usize = 4
```

## 매개변수, 반환값, 일반적인 사용법

값이 4인 Rust usize 상수로, Go 호환 바이너리 전송 형식의 빅엔디언 소수 자릿수 접두부 바이트 길이입니다. 십진 숫자 개수나 정밀도 한계가 아닙니다. 브라우저는 Rust에서 가져온 읽기 전용 number를 공개합니다. 크레이트 루트 재내보내기는 Rust 가져오기 문법입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-precision-fixed-size" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
