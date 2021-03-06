//! Peripheral access API for STM32F4 microcontrollers
//! (generated using [svd2rust])
//! [svd2rust]: https://github.com/japaric/svd2rust
//!
//! You can find an overview of the API here:
//! https://docs.rs/svd2rust/0.12.1/svd2rust/#peripheral-api
//!
//! For more details see the README here:
//! https://github.com/adamgreig/stm32-rs

#![allow(non_camel_case_types)]
#![allow(private_no_mangle_statics)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]

#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]

extern crate vcell;
extern crate bare_metal;
extern crate cortex_m;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::{default_handler, exception};

#[cfg(feature = "stm32f401")]
pub mod stm32f401;

#[cfg(feature = "stm32f405")]
pub mod stm32f405;

#[cfg(feature = "stm32f407")]
pub mod stm32f407;

#[cfg(feature = "stm32f410")]
pub mod stm32f410;

#[cfg(feature = "stm32f411")]
pub mod stm32f411;

#[cfg(feature = "stm32f412")]
pub mod stm32f412;

#[cfg(feature = "stm32f413")]
pub mod stm32f413;

#[cfg(feature = "stm32f427")]
pub mod stm32f427;

#[cfg(feature = "stm32f429")]
pub mod stm32f429;

#[cfg(feature = "stm32f446")]
pub mod stm32f446;

#[cfg(feature = "stm32f469")]
pub mod stm32f469;

