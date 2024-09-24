#![no_std]
pub mod time_driver;

pub const EXTCLK_FREQ: u32 = 40_000_000;

pub use time_driver::init;
