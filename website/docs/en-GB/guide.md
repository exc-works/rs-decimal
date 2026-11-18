---
locale: en-GB
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# Getting started

Construct decimal values from strings when the input must remain exact. Multiplication with `mul_exact` preserves the sum of input scales; `string()` omits trailing zeros while `string_with_trailing_zeros()` retains them.

<<< ../../../examples/web_getting_started.rs

Precision arguments count decimal places, not significant digits. `HalfEven` resolves midpoint ties toward an even retained digit. `Down` truncates toward zero; it does not mean negative infinity.

`Decimal::default()` is uninitialised numerical zero. An initialised zero is numerically equal but has different state. `NullDecimal.valid` independently determines whether a value is present.

Ordinary fallible methods return `Result`. Check the error category; only explicitly named `must_*` methods panic for expected input failures. Browser calls will use the same Rust methods through a JavaScript bridge, with documented type and ownership mappings.
