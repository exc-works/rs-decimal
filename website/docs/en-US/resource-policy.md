# Playground resource policy

The browser bridge has a deliberately smaller domain than the Rust library.
A `BridgeError` with `category: "resource"` means a playground budget was reached;
it is not a numerical-library error. Rust method names, parameter order and
results within these limits remain unchanged. The core crate is not modified.

| Bound | Current value |
| --- | --- |
| Decimal input | 8192 UTF-8 bytes, 4096 mantissa/expanded coefficient digits |
| Stored scale and input scale magnitude | 1000 decimal places |
| Estimated result coefficient | 16384 bits (at most 4933 decimal digits) |
| ByteBuffer input | 65536 bytes |
| Calls entering a Wasm instance | 10000, including getters, snapshots and metadata |
| Built Wasm linear memory maximum | 64 MiB |

Input scale and positive exponent expansion are checked before Rust parsing.
Exact products check the sum of scales; rounded products check the output scale.
Both bound intermediate coefficient bits before multiplying. Addition/subtraction
check scale-alignment growth and a carry; division checks numerator or denominator
scaling before allocation. Quotient/remainder checks alignment after zero-divisor
validation. Powers bound normalized intermediate scale and coefficient growth,
including reciprocal scaling; core zero/unit fast paths and pre-allocation
Overflow remain intact. Owned arithmetic consumes its receiver even when this
preflight fails. Exact rational conversion checks reduced denominator factors
and coefficient growth; rounded rational conversion checks numerator scaling
before multiplication. Nonzero-denominator validation precedes rational construction.
Negative precision and zero divisors retain their core
errors when the core can reject them before allocation.
Rescale checks the requested scale and a conservative growth estimate before
allocating powers of ten. Precision-specific rounding uses the same checks.
Shift checks both the resulting scale and coefficient growth; significant-figure
rounding reserves a possible carry. Core shift overflow is retained when Rust
can return it before allocation. Consequently, a sequence of accepted operations cannot
produce an unbounded coefficient or an unbounded formatted decimal string.
Conservative preflight can reject an operation whose final value would be smaller
(for example a product with a zero operand); the limit is not a core-library rule.
Ordinary malformed text, negative precision and unknown rounding modes retain
library errors when they do not independently violate a resource bound.

A consuming method still consumes its value when Rust rejects a resource bound.
Borrowed operands remain unchanged. After the per-instance budget is exhausted,
all exported calls keep failing except disposal; creating another JS facade does
not reset it. Starting a fresh Wasm instance is the reset boundary.

The JS facade also checks string length and buffer length before wasm-bindgen
copies the arguments. Rust rechecks its incoming slice before core parsing or
additional allocation. Direct raw ABI callers can allocate the temporary input
copy before the Rust check, but cannot remove the built module's memory maximum.
The runner must keep that raw ABI private. This is not a hard quota for JavaScript
heap or browser process memory. The runner enforces Worker termination, source/output limits and network isolation.

A local Node 26 run parsed/read a 4096-digit coefficient and formatted a value at
scale 1000 in roughly 1.2 ms; this is an indicative development measurement, not a
latency promise. CI prints the same measurement and checks rejection boundaries.
The development Wasm artifact was about 196 KB (73 KB gzip); `build.json` records
the final commit's exact size, hashes and memory maximum.
