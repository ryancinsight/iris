# ADR 0002: Consumer color-law consolidation

- Status: Accepted
- Date: 2026-07-21
- Class: `[arch]` `[minor]`

## Context

CFDrs defines a public three-variant color-map enum and local blue-red,
grayscale, and approximate Viridis formulas. Iris already owns grayscale and
Viridis, but it does not own CFDrs's exact endpoint-polarity law. Kwavers also
retains local map tables, but its renderer is under an active branch claim and
is outside this increment.

## Decision

Add the exact law `c(t) = (t, 0, 1 - t, 1)` as the zero-sized `BlueRed`
strategy and as a `NamedColorMap` variant. CFDrs consumes `NamedColorMap`
directly and removes its parallel enum and formulas. Consumer-specific field
interpretation, scalar-range reduction, byte-color representation, and
Plotters rendering remain in CFDrs.

The map is additive to Iris. Adding it to the non-exhaustive runtime enum does
not break external exhaustive matching. The new variant is appended so every
existing implicit discriminant remains stable. Static consumers monomorphize
through `BlueRed: ColorMap`; runtime-selected consumers dispatch once through
`NamedColorMap`.

## Rejected alternatives

- Reuse `CoolWarm`: rejected because its white midpoint changes the existing
  CFDrs endpoint-polarity contract.
- Keep a CFDrs wrapper enum: rejected because it preserves duplicate ownership
  and requires ongoing variant mapping.
- Move CFD range calculation into Iris: rejected because this increment has no
  second consumer contract for range policy, non-finite data, or degenerate
  fields.

## Proof obligations

For every `t` in `[0, 1]`, `t` and `1 - t` lie in `[0, 1]`; the remaining
channels are `0` and `1`. The RGBA invariant follows directly. For ordered
`t1 <= t2`, red is monotone non-decreasing and blue is monotone
non-increasing. On the uniform byte grid, nearest-byte quantization preserves
`red + blue = 255`.

## Verification

- exhaustive 1,025-point finite/range checks through `NamedColorMap::ALL`;
- all 256 byte coordinates checked for channel complement and monotonicity;
- exact endpoint assertions;
- zero-sized strategy and lookup-table layout checks remain active;
- differential CFDrs tests compare the public byte-color boundary.

## Outcome

Iris PR 3 merged the provider law as `ef43861a`. CFDrs PR 303 merged the
direct consumer migration as `394c9977`, deleting its local map enum and
formulas. CFDrs computes scalar ranges once per overlay, borrows existing
field maps through `Cow`, and performs constant-time color lookup without a
per-element range allocation. Atlas owns the independent parent-gitlink
reconciliation.
