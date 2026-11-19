---
locale: en-GB
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# Getting started

A customer orders two notebooks at 4.50 each. Delivery costs 1.25. Let’s work out what to charge and print the amount on the receipt.

## Work out the total

Start with the price as written on the order. `new_from_string("4.50")` reads those digits directly. Multiply the price by two with `mul_exact` to get 9.00, then add delivery with `add` to get 10.25.

<<< ../../../examples/web_getting_started.rs#calculate

## Print the receipt

For the subtotal, `string()` gives "9" and `string_with_trailing_zeros()` gives "9.00". Both represent the same amount. The second keeps the two decimal places from the price, as you would expect on a receipt.

<<< ../../../examples/web_getting_started.rs#print

[Try the receipt in your browser](./playground) — Try changing the quantity from 2 to 3. The subtotal should become 13.50 and the amount due 14.75.

## Run the Rust example locally

With Rust 1.85 or later, run these commands to download the repository and run its example. It prints 10.25.

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
