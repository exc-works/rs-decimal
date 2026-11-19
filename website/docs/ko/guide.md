---
locale: ko
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# 시작하기

고객이 단가 4.50인 공책 두 권을 주문했습니다. 배송비는 1.25입니다. 얼마를 받아야 하는지 계산하고 영수증에 표시해 봅시다.

## 결제 금액 계산하기

주문서에 적힌 단가를 `new_from_string("4.50")`으로 읽습니다. `mul_exact`로 수량 2를 곱하면 상품 소계는 9.00입니다. 여기에 `add`로 배송비를 더하면 결제 금액은 10.25가 됩니다.

<<< ../../../examples/web_getting_started.rs#calculate

## 영수증에 금액 표시하기

소계에 `string()`을 호출하면 "9", `string_with_trailing_zeros()`를 호출하면 "9.00"이 나옵니다. 같은 금액이지만 두 번째 방법은 단가의 소수점 아래 두 자리를 영수증에도 유지합니다.

<<< ../../../examples/web_getting_started.rs#print

[브라우저에서 수량 바꿔 보기](./playground) — 수량을 2에서 3으로 바꾸고 다시 실행해 보세요. 소계는 13.50, 결제 금액은 14.75가 됩니다.

## 내 컴퓨터에서 Rust 예제 실행하기

Rust 1.85 이상에서 다음 명령으로 저장소를 내려받고 예제를 실행하세요. 터미널에 10.25가 출력됩니다.

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
