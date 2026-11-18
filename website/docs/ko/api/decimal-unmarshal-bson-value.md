---
locale: ko
source_revision: d77e5dd32cf21e2ddfd9d7df9654834b1090bc84b953f82d973c756dd33fd206
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::unmarshal_bson_value"
---
# Decimal::unmarshal_bson_value

BSON 스칼라를 기존 값으로 디코딩합니다.

```rust
fn unmarshal_bson_value (& mut self , kind : u8 , data : & [u8]) -> Result < () , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

kind: u8/number, data: &[u8]/Uint8Array. 수신 객체를 바꾸고 `Result<(), DecimalError>`/undefined를 반환합니다. Rust는 bson이 필요합니다. double (1), string (2), null (10), int32 (16), int64 (18), Decimal128 (19)를 지원하며 숫자 페이로드는 BSON 리틀엔디언 바이트를 사용합니다. int64에는 JS number 정밀도 손실을 피하도록 원시 바이트를 전달하세요. BSON null은 수신 객체를 미초기화 0으로 재설정하며, 변경하지 않는 unmarshal_json(null)과 다릅니다. null이 아닌 입력의 nullable 디코딩이 성공하면 valid=true로 설정합니다. 미지원 종류, 잘린 데이터, 비유한 숫자는 부분 변경 없이 실패합니다. 호환성 주의점으로 문자열 디코딩은 선언된 마지막 바이트를 NUL인지 확인하지 않고 건너뛰며 필요한 페이로드 이후 바이트는 무시합니다. Decimal128은 Rust 코어가 디코딩하며 지원하는 소수 정밀도를 유지하지만 marshal_bson_value 출력은 여전히 문자열입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-unmarshal-bson-value" locale="ko" />

## 브라우저 제한

입력은 Uint8Array, kind는 0–255 정수여야 합니다. Wasm 복사 전에 입력을 65536바이트로 제한합니다. Rust는 바이너리 소수 자릿수를 최대 1000, 실제 계수를 최대 16384비트로 검사합니다. BSON 문자열은 8192바이트 텍스트 제한을 따르며 Decimal128 지수 확장도 [리소스 검사](../resource-policy)를 받습니다. 네이티브 라이브러리보다 좁은 제한일 수 있습니다. 검사는 수신 객체 변경 전에 수행하고 코어 오류와 리소스 오류를 구분하며 이동된 핸들은 재사용할 수 없습니다.

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
