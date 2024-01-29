# polyeval

> Evaluate polynomials.

[![crates.io](https://img.shields.io/crates/v/polyeval?logo=rust)](https://crates.io/crates/polyeval)
[![docs.rs](https://img.shields.io/docsrs/polyeval?logo=docsdotrs)](https://docs.rs/polyeval)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/polyeval&color=brightgreen&logo=github)](https://github.com/FedericoStra/polyeval)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/polyeval/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/polyeval/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/polyeval/status.svg)](https://deps.rs/repo/github/FedericoStra/polyeval)
[![MIT license](https://img.shields.io/crates/l/polyeval)](https://choosealicense.com/licenses/mit/)

## Contents

This crate provides four macros (`horner!`, `horner_fma!`, `estrin!`, and `estrin_fma!`) to evaluate
a polynomial using either [Horner's method] or [Estrin's scheme].
The `_fma` variants use "fused multiply-add" instructions where applicable.

[Horner]: https://en.wikipedia.org/wiki/Horner%27s_method
[Estrin]: https://en.wikipedia.org/wiki/Estrin%27s_scheme

## Other crates

On <https://crates.io> there are several crates related to polynomial evaluation:

- [`horner`](https://crates.io/crates/horner): provides two functions to evaluate polynomials of known and unknown order;
- [`horner-eval`](https://crates.io/crates/horner-eval): provides a macro to evaluate polynomials with known coefficients and a function to evaluate polynomials of unknown order;
- [`fast_polynomial`](https://crates.io/crates/fast_polynomial): provides two functions to evaluate polynomials of known and unknown order, implementing a hybrid Estrin's/Horner's method exploiting instruction-level parallelism;
- [`polynomen`](https://crates.io/crates/polynomen): general arithmetic with polynomials (possibly incomplete?);
- [`horny`](https://crates.io/crates/horny) and [`horny_macro`](https://crates.io/crates/horny) are "Reserved for future use (seriously)", but are completely empty and haven't received an update in 3+ years.
