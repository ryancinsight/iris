# Exact nearest-bin quantization

`Rgba::to_rgba8` and `LookupTable::sample` share the same uniform quantizer.
For a validated binary32 coordinate `v` and `N` bins, its integer result is
`floor((N - 1) * v + 1/2)`, evaluated on the exact value represented by `v`.
Midpoint ties choose the higher bin. Bytes use `N = 256`; lookup tables retain
their structural range `2 <= N <= 16_777_217`.

## Why a floating-point offset is insufficient

Adding `0.5` to a binary32 product can round to an integer before the cast.
For example, `f32::from_bits(0x3eff_ffff)` is immediately below `0.5`, yet
adding `0.5` rounds to `1.0`. A two-entry table therefore used to select entry
one below the midpoint. For byte channels, `f32::from_bits(0x3b00_8080)` has
exact scaled value `255v = 16_777_215 / 33_554_432`, which is below one half,
but the old conversion returned byte one. Rounding the floating-point product
without the offset would still permit the product itself to round onto a tie.

At the largest supported table size, `f32::from_bits(0x3f00_0001)` scales
exactly to index `8_388_609`. Adding `0.5` in binary32 instead rounds that odd
integer to `8_388_610`. This failure affects integral indices as well as bin
boundaries.

## Integer derivation and bounds

For a positive normal binary32 input, let `e` be the biased exponent and `s`
the 24-bit significand, including its implicit leading bit. Then
`v = s / 2^(150 - e)`. The codec forms the exact integer product
`p = s * (N - 1)` in `u64`. Since `s < 2^24` and `N - 1 <= 2^24`,
`p < 2^48`.

Write `k = 150 - e`. If `k >= 49`, the scaled coordinate is strictly below
one half, so its nearest index is zero. Every subnormal, either signed zero
and every smaller normal input belongs to this case. Otherwise
`23 <= k <= 48`, because the validated input is at most one.

The branch-free implementation sets `h = min(k - 1, 48)` and returns
`((p >> h) + 1) >> 1`. For a normal input outside the tiny case, this is
`floor((floor(2 * (N - 1) * v) + 1) / 2)`, exactly the nearest quantizer with
upward ties. In the tiny case, shifting any product below `2^48` by 48 gives
zero. This remains true for subnormal/zero encodings even though their
implicit-leading-bit interpretation is inapplicable. The cap deliberately
discards that constructed product, rather than relying on it as a decoded
subnormal value. The shifted value is at most `2 * (N - 1)`; adding one fits
`u64`, all shifts are between 22 and 48, and the result lies in `[0, N - 1]`,
which fits `usize`. The byte specialization fits `u8` without saturation.

The implementation uses one const-resolution codec and performs no allocation
or floating-point widening. Construction still samples each map at the rounded
binary32 coordinates of the uniform grid; lookup indexing quantizes against
the ideal uniform grid. No palette value, interpolation law, storage layout,
feature or public signature changes.

## Independent verification

The regression suite includes both public failure witnesses, all 255 byte
midpoints and two representable neighbors on each side, twelve odd/even table
resolutions, exact ties, endpoints, subnormals and signed zero. A deterministic
bit-pattern sweep covers the binary32 exponent range. Private codec tests
exercise the largest resolution without allocating its table; compile-fail
doctests preserve the structural size bounds.

The independent test oracle multiplies the input by the integer scale in
binary64, then calls its nearest-rounding operation. This product is exact:
the two factors have at most 24 significant bits each, requiring at most 48
bits, within binary64's 53-bit significand. Its exponent range also contains
every permitted input and product. This widening belongs only to the oracle;
production computes the integer formula above.

These checks establish quantization correctness. They make no continuous
palette-fidelity or whole-render performance claim.
