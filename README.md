[![Crates.io](https://img.shields.io/crates/v/va416xx)](https://crates.io/crates/va416xx)
[![build](https://github.com/us-irs/va416xx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/us-irs/va416xx-rs/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/va416xx)](https://docs.rs/va416xx)

# PAC for the Vorago VA416xx microcontroller family

This repository contains the Peripheral Access Crate (PAC) for
Voragos VA416xx series of Cortex-M4 based microcontrollers.

The crate was generated using [`svd2rust`](https://github.com/rust-embedded/svd2rust).

## Usage

To use this crate, add this to your `Cargo.toml`

```toml
[dependencies.va108xx]
version = "0.1"
features = ["rt"]
```

The `rt` feature is optional and recommended. It brings in support for `cortex-m-rt`.

For full details on the autgenerated API, please see the
[svd2rust documentation](https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api).

## Regenerating the PAC

You can regenerate the PAC by running the `gen-helper.sh` helper script.
