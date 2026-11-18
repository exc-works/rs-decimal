---
locale: ko
source_revision: b225904d6e477d16ac1df6ba10bcfee3249227a343581851ba1cc46cc84f5863
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_binary"
---
# Decimal::marshal_binary

Go 호환 precision/Gob 전송 형식으로 인코딩합니다.

```rust
fn marshal_binary (& self) -> Result < Vec < u8 > , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Result<Vec<u8>, DecimalError>`/Uint8Array를 반환합니다. 초기화된 값은 소수 부분의 후행 0을 제거한 뒤 4바이트 빅엔디언 소수 자릿수, big.Int Gob 버전/부호 바이트, 빅엔디언 절댓값을 씁니다. 소수 스칼라 페이로드이며 IEEE 실수가 아닙니다. 미초기화 0은 빈 바이트, 초기화된 0은 [0,0,0,0,2]입니다. 반환된 복사본을 바꿔도 Decimal은 변하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-binary" locale="ko" />

## 브라우저 제한

입력은 Uint8Array, kind는 0–255 정수여야 합니다. Wasm 복사 전에 입력을 65536바이트로 제한합니다. Rust는 바이너리 소수 자릿수를 최대 1000, 실제 계수를 최대 16384비트로 검사합니다. BSON 문자열은 8192바이트 텍스트 제한을 따르며 Decimal128 지수 확장도 [리소스 검사](../resource-policy)를 받습니다. 네이티브 라이브러리보다 좁은 제한일 수 있습니다. 검사는 수신 객체 변경 전에 수행하고 코어 오류와 리소스 오류를 구분하며 이동된 핸들은 재사용할 수 없습니다.

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
