---
locale: en-US
source_revision: 637474c992943a95b792c86f50bfa04821ba9ba29fe9dab7696d42ada69cbe84
---
# Playground resource policy

The Playground reports `category: "resource"` when a calculation exceeds one of the limits below.

| Bound | Current value |
| --- | --- |
| Decimal input | 8192 UTF-8 bytes, 4096 mantissa/expanded coefficient digits |
| Stored scale and input scale magnitude | 1000 decimal places |
| Estimated result coefficient | 16384 bits (at most 4933 decimal digits) |
| Binary/BSON and ByteBuffer input | 65536 bytes |
| Validation rule/error lists and message maps | 64 entries; 8192 bytes per string; 65536-byte JSON transport |
| Validation substituted text | conservative 65536-byte total before translation |
| Calls entering a Wasm instance | 10000, including getters, snapshots and metadata |
| Nontrivial root degree | 128 |
| Log/exp output precision, input scale, input coefficient | 128 places, 256 places, 4096 bits |
| Ordinary exp absolute input | 64 (proven negative round-to-zero fast path retained) |
| Built Wasm linear memory maximum | 64 MiB |

Input scale and positive exponent expansion are checked before Rust parsing.
Configuration conversions check original decimal text and decoded JSON strings;
128-bit integer variants enforce exact signed/unsigned bounds before transport.
Generic Serde adapters retain their own error surface and reject float visitors.
SQL scanning checks text after core-compatible outer quote removal. Validation
checks decimal rule parameters before construction and bounds repeated placeholder
expansion before translation; fields and inserted parameters remain literal text.
Text/parameter/JSON decoders cap source bytes at 8192 and check decoded decimal
text before mutation, including JSON escapes. Original JSON numeric bytes stay
exact; the browser never parses them through JavaScript numbers.
Binary decoding bounds actual coefficient bits and encoded scale before BigInt
allocation. YAML/XML source is capped at 8192 bytes; decoded scalar/element text
is checked before core numeric parsing, including escapes, CDATA, YAML underscore
normalization and float zero insertion. YAML scalar radix allocation is bounded
by a conservative 4096 hexadecimal digit character limit.
BSON strings use the text limits; Decimal128 exponent expansion is
checked before core parsing. All decoder preflight failures preserve receiver state.
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
Formatting bounds width (65536 bytes), precision (1000) and fixed-point coefficient
growth. Append checks the existing buffer plus a conservative decimal text bound
before mutation; even a just-fitting operation may be rejected.
Roots bound scaled values, Newton powers and exact midpoint certification before
calling the core. Nontrivial degrees are at most 128; zero, unit and degree-one
fast paths remain available within the output scale budget.
Log/exp have tighter input and precision budgets to bound rational reduction and
all interval refinement retries, including the core maximum of 8192 guard bits.
Exp additionally bounds ordinary argument magnitude; sufficiently negative input
that the core proves rounds to zero keeps its safe fast path.
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
