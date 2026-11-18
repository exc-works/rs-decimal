---
locale: ko
source_revision: 40283a0b4501fabd3b307e9ac7d1a0dd292a797503fe33899ba506e5b43fdac8
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

## 브라우저 제한

입력은 Uint8Array, kind는 0–255 정수여야 합니다. Wasm 복사 전에 입력을 65536바이트로 제한합니다. Rust는 바이너리 소수 자릿수를 최대 1000, 실제 계수를 최대 16384비트로 검사합니다. BSON 문자열은 8192바이트 텍스트 제한을 따르며 Decimal128 지수 확장도 [리소스 검사](../resource-policy)를 받습니다. 네이티브 라이브러리보다 좁은 제한일 수 있습니다. 검사는 수신 객체 변경 전에 수행하고 코어 오류와 리소스 오류를 구분하며 이동된 핸들은 재사용할 수 없습니다.

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
