//! # API for the GPIO peripheral
//!
//! The implementation of this GPIO module is heavily based on the
//! [ATSAMD HAL implementation](https://docs.rs/atsamd-hal/latest/atsamd_hal/gpio/index.html).
//!
//! This API provides two different submodules, [pin] and [dynpin],
//! representing two different ways to handle GPIO pins. The default, [pin],
//! is a type-level API that tracks the state of each pin at compile-time. The
//! alternative, [dynpin] is a type-erased, value-level API that tracks the
//! state of each pin at run-time.
//!
//! The type-level API is strongly preferred. By representing the state of each
//! pin within the type system, the compiler can detect logic errors at
//! compile-time. Furthermore, the type-level API has absolutely zero run-time
//! cost.
//!
//! If needed, [dynpin] can be used to erase the type-level differences
//! between pins. However, by doing so, pins must now be tracked at run-time,
//! and each pin has a non-zero memory footprint.
//!
//! ## Examples
//!
//! - [Blinky example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/blinky.rs)
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("pin is masked")]
pub struct IsMaskedError;

pub mod pin;
pub use pin::*;

pub mod dynpin;
pub use dynpin::*;

mod reg;
