# Scalar-field zero cardinality

IRIS-016 is a patch correction to `ScalarFieldView::new` in the independent
0.1.2 candidate based on current 0.1.1 main. No array-layout provider, renderer,
color law, public type, dependency, stride/offset policy or publication setting
changes. Independent exact-head review and existing release/authentication
acceptance remain required before merge/publication.

## Contract and proof

For positive rank D, let P be the exact nonnegative integer product of the
extents. Construction accepts exactly when P equals the borrowed slice length;
it reports overflow when nonzero P exceeds usize::MAX and otherwise reports the
exact expected/actual mismatch. A zero-sized element type does not make a
nonempty logical slice empty. Rank zero remains a compile-time error.

The previous checked fold can stop before a later zero: [MAX,2,0] rejects an
empty slice, while [0,MAX,2] accepts it. Both have exact cardinality zero.

The fix keeps the original checked fold. Only after a prefix overflows does it
inspect the whole shape for a zero extent. A zero makes the exact total zero;
without a zero all factors are positive, so later multiplication cannot reduce
an overflowing prefix into the representable range. Thus every nonzero
overflow still fails, every representable product retains its original value,
and zero-cardinality validation becomes independent of dimension order. No
wrapping multiplication, saturation, huge allocation or scalar conversion is
introduced. Validation is O(D), allocation-free, and occurs once at construction.
The slice, shape and iteration representation are unchanged.

## Layout boundary

Iris stores only a borrowed contiguous slice and its shape. It has no signed
stride or physical-offset metadata to validate. Current Leto's logical
`shape_size` already returns zero for any zero extent. Its separate C/F
contiguous builders still check conversion to signed dimensions and signed
stride multiplication; these representation checks may reject huge empty
layouts. Leto also admits rank-zero scalar layouts, whereas Iris requires
positive rank. This Iris fix neither infers a universal empty-tensor policy nor
changes any of those provider contracts.

## Reproductions and oracle

The frozen initial audit has three failures and six controls on unchanged main.
Expanded baseline has six failing tests and nine controls in debug, release and
no-default mode. All fifteen pass after the repair. Tests retain the original
failures, all twenty-four rank-four index permutations (including repeated zero
extents), genuine nonzero overflow in every ordering, the exact MAX-1 boundary,
nonempty zero-sized-element slices, borrow identity and ordinary computed
values. A compile-fail doctest retains rank-zero rejection.

An actual old/new library caller checks 19,120 runtime inputs per standard
O2/O3 profile across ranks 1,2,3,4,6,16 and five slice lengths. An external
arbitrary-precision product is the reference. Per profile, 15,370 outputs retain
legacy results, 750 formerly rejected empty views recover, and 3,000 nonempty
zero-cardinality inputs report the exact mismatch instead of overflow. All
8,140 genuinely overflowing nonzero cases still report overflow. Successful
views verify shape, slice pointer and iterator length; profile outputs match
byte-for-byte. The corpus is bounded, not exhaustive across ranks/platforms.

## Caller and resource evidence

One ARM64 Mac, Rust 1.97, native usize64, O2/O3, repository release overflow
checks unchanged. Actual constructors return complete black-boxed Results in
direct, opaque indirect and runtime 1,024-input streaming callers. Seven
alternating old/new samples use 200,000 calls, or 256 streaming batches. Setup
and validation are excluded; local builds/tests finish before timing.

| Profile | Rank | Baseline ns/call range | Candidate ns/call range | Paired delta ns range |
|---|---:|---:|---:|---:|
| O2 | 2 | 0.613 to 1.037 | 0.671 to 1.037 | -0.001 to 0.175 |
| O2 | 3 | 0.873 to 1.288 | 1.036 to 1.559 | 0.127 to 0.271 |
| O2 | 6 | 1.214 to 1.533 | 1.221 to 1.797 | 0.008 to 0.264 |
| O2 | 16 | 6.404 to 9.228 | 6.403 to 9.373 | -0.000 to 0.145 |
| O3 | 2 | 0.665 to 1.027 | 0.641 to 1.038 | -0.024 to 0.010 |
| O3 | 3 | 0.838 to 1.298 | 1.036 to 1.533 | 0.111 to 0.235 |
| O3 | 6 | 1.206 to 1.558 | 1.288 to 1.796 | 0.030 to 0.239 |
| O3 | 16 | 6.603 to 9.568 | 6.731 to 9.572 | -0.035 to 0.128 |

Paired ordinary changes span -0.035 to +0.272 ns/call (-3.54% to +23.55%) in
these contexts. These small negative differences establish no general speedup.
A correct eager zero pre-scan variant had sampled overhead up to 0.77 ns; the
final lazy check avoids that unconditional scan and reduces that sampled maximum,
without claiming every caller improves or any cost is unavoidable. Both complete
measurement sets and standard disassembly are retained for independent review.

Every one of the 630 samples/profile requests zero allocation calls/bytes.
Measured Result object sizes stay 32/40/64/144 bytes at ranks 2/3/6/16; these
are object sizes, not peak stack, RSS, cache or whole-render guarantees. Old
recovered-empty outputs are wrong and are not throughput targets. Caller-cost
acceptance remains explicit independent-review work; no compiler threshold,
forced-inline attribute, limit, lint or test reduction is used.
