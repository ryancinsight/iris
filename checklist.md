# Checklist

## IRIS-001

- [x] Confirm the Atlas promotion gate and absence of an existing Iris repository.
- [x] Audit duplicated visualization laws and define ownership/non-goals in ADR 0001.
- [x] Implement normalized colors, ZST maps, fixed lookup tables, borrowed views, and render seam.
- [x] Pass format, Clippy, 14/14 Nextest, two doctests, warning-clean Rustdoc,
      feature-seam, cargo-deny, example, and package gates.
- [x] Publish the public repository and verify anonymous access at merged
      revision `e2edd476`.
- [x] Migrate `ritk-snap` and `ritk-vtk`; delete their duplicated color
      computations through RITK PR 46 and close exact-head verification in
      PR 47 at `a36e65df`.
- [x] Register Iris and the initial merged RITK consumer revision through
      Atlas PR 71 at `6740296b`.
