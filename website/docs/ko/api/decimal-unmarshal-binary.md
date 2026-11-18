---
locale: ko
source_revision: 404c61af98ea51d383b516ba1e691c0c718c44923614dbeaca2d2a7efd410ffc
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_binary"
---
# Decimal::unmarshal_binary

Go 호환 바이너리 스칼라로 소수를 교체합니다.

```rust
fn unmarshal_binary (& mut self , bytes : & [u8]) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

bytes: &[u8]/Uint8Array. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. 첫 4바이트는 빅엔디언 소수 자릿수이며 Gob 헤더 2(양수) 또는 3(음수)와 절댓값이 이어집니다. 성공하면 인코딩된 자릿수를 유지합니다. 빈 입력은 기본 미초기화 0과 달리 초기화된 0을 만듭니다. Gob 페이로드가 없는 4바이트 접두부도 해당 자릿수의 초기화된 0으로 수락합니다. 짧은 접두부, 잘못된 버전, 131072를 넘는 자릿수는 코어 Unmarshal 오류를 반환합니다. 오류는 수신 객체를 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-binary" locale="ko" />

## 브라우저 제한

입력은 Uint8Array, kind는 0–255 정수여야 합니다. Wasm 복사 전에 입력을 65536바이트로 제한합니다. Rust는 바이너리 소수 자릿수를 최대 1000, 실제 계수를 최대 16384비트로 검사합니다. BSON 문자열은 8192바이트 텍스트 제한을 따르며 Decimal128 지수 확장도 [리소스 검사](../resource-policy)를 받습니다. 네이티브 라이브러리보다 좁은 제한일 수 있습니다. 검사는 수신 객체 변경 전에 수행하고 코어 오류와 리소스 오류를 구분하며 이동된 핸들은 재사용할 수 없습니다.

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
