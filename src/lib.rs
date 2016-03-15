#![no_std]
#![crate_type="staticlib"]

#![feature(lang_items)]
#![feature(core_intrinsics)]

//#[cfg(target_os = "none")]
extern crate rlibc;

pub mod runtime;
mod memory;
pub mod hal;
pub mod driver;
mod debug;
pub mod systick;

