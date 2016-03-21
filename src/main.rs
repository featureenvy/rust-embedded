#![no_std]
#![crate_type="staticlib"]

#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(start)]

//#[cfg(target_os = "none")]
extern crate rlibc;

pub mod runtime;
mod memory;
pub mod hal;
pub mod driver;
mod debug;
pub mod systick;
mod labs;

use labs::*;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // lab4::run();
    // lab5::run();
    // lab6::run();
    // lab7::run();
    // lab8::run();
    // lab9::run();
    lab10::run();

    0
}
